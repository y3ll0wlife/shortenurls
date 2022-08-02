use auth::{authorize, create_user};
use regex::Regex;
use std::collections::HashMap;
use structs::{SlugPayload, SlugValue, UserPayload, UserValueResponse};
use utils::log_request;
use worker::*;

mod auth;
mod structs;
mod utils;

const KV_NAME: &str = "slugs";
const CF_SECRET_HASH: &str = "SECRET_HASH";
const CF_GLOBAL_ADMIN_KEY: &str = "GLOBAL_ADMIN_KEY";

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    utils::set_panic_hook();

    let router = Router::new();

    router
        .get("/", |_, _| {
            Response::ok("https://github.com/y3ll0wlife")
        })
        .post_async("/user", |mut req, ctx| async move {
            let payload = req.json::<UserPayload>().await;
            match payload {
                Ok(body) => {
                    let token_str = match req.headers().get("authorization") {
                        Ok(token) => match token {
                            Some(t) => t,
                            None => return Response::error(
                                "bad request missing authorization header",
                                400,
                            ),
                        },
                        Err(_) => {
                            return Response::error(
                                "bad request missing authorization header",
                                400,
                            )
                        }
                    };

                    if token_str != ctx.secret(CF_GLOBAL_ADMIN_KEY)?.to_string() {
                        return Response::error(
                            "unauthorized",
                            401,
                        )
                    }

                    let token = create_user(&body.username, ctx.secret(CF_SECRET_HASH)?.to_string());

                    let response = UserValueResponse {
                        token,
                        username: body.username
                    };

                    Response::from_json(&response)
                   
                }
                Err(_) => Response::error(
                    "bad request request has to be of 'Content-Type' 'application/json'",
                    400,
                ),
            }
        })
        .post_async("/slugs", |mut req, ctx| async move {
            let payload = req.json::<SlugPayload>().await;
            match payload {
                Ok(body) => {
                    let auth = authorize(&req,ctx.secret(CF_SECRET_HASH)?.to_string());

                    if !auth.success {
                        return Response::error("unauthorized", 401)
                    }

                    let blacklisted_slugs = ["slugs", "user"];

                    if body.name.len() <= 0 {
                        return Response::error(
                            "bad request value of 'name' length has to be greater then or equal to 1",
                            400,
                        );
                    }

                    if blacklisted_slugs.contains(&body.name.as_str()) {
                        return Response::error(
                            "bad request value of 'name' is a blacklisted term",
                            400,
                        );
                    }

                    let url_regex = Regex::new(r"^https?://(www.)?[-a-zA-Z0-9@:%._+~#=]{1,256}.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_+.~#?&//=]*)$").unwrap();
                    if !url_regex.is_match(body.url.as_str()){
                        return Response::error(
                            "bad request invalid url",
                            400,
                        );
                    }

                    let kv = ctx.kv(KV_NAME)?;

                    let store_value = SlugValue  {
                        url: body.url,
                        creator: auth.username,
                        created_at: Date::now().to_string()
                    };
                    
                    kv.put(&body.name, store_value)?.execute().await?;

                    Response::empty()
                   
                }
                Err(_) => Response::error(
                    "bad request request has to be of 'Content-Type' 'application/json'",
                    400,
                ),
            }
        })
        .get_async("/slugs", | req, ctx| async move {
            if !authorize(&req, ctx.secret(CF_SECRET_HASH)?.to_string()).success {
                return Response::error("unauthorized", 401)
            }
           
            
            let kv = ctx.kv(KV_NAME)?;
            let slugs = kv.list().execute().await?;

            let keys = slugs.keys.iter().map(|k| k.name.clone()).collect::<Vec<_>>();
            let mut payload = HashMap::new();

            for key in &keys  {
                let value =  match kv.get(key).json::<SlugValue>().await? {
                    Some(body) => body,
                    None => continue,
                };
             
                payload.insert(
                    key,
                    value,
                );
            }

            Response::from_json(&payload)
        })
        .get_async("/:slug", |_req, ctx| async move {
            let slug = ctx.param("slug");
            match slug {
                Some(slug_id) => {
                    let kv = ctx.kv(KV_NAME)?;
                    let value = kv.get(slug_id).json::<SlugValue>().await?;

                    if let Some(body) = value {
                        let url = Url::parse(&body.url)?;
                        return Response::redirect(url)
                    };

                    Response::error("404 does not exist", 404)
                }
                None => Response::error("bad request", 400),
            }
        })
        .delete_async("/:slug", |req, ctx| async move {
            if !authorize(&req, ctx.secret(CF_SECRET_HASH)?.to_string()).success {
                return Response::error("unauthorized", 401)
            }

            let slug = ctx.param("slug");
            match slug {
                Some(slug_id) => {
                    let kv = ctx.kv(KV_NAME)?;
                    kv.delete(slug_id).await?;
                   
                    Response::empty()
                }
                None => Response::error("bad request", 400),
            }
          
        })        
        .run(req, env)
        .await
}
