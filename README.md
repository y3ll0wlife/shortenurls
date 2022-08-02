# Shortenurls
### Using Cloudflare Workers and KV


## API Documentation
POST **/slugs**
> Create a new slug

Header of Authorization with JWT
```json
{
	"name": "github",
	"url": "https://github.com/y3ll0wlife"
}
```
Returns 200 OK on success
Returns 401 on unauthorized request

---
GET **/slugs**
> Get a hashmap of the slugs and the metadata

Header of Authorization with JWT

Returns 200 OK on success with body of
```json
{
	"github": {
		"url": "https://github.com/y3ll0wlife",
		"creator": "y3ll0w",
		"created_at": "Tue Aug 02 2022 08:57:16 GMT+0000 (Coordinated Universal Time)"
	},
    "discord": {
		"url": "https://discord.com/users/190160914765316096",
		"creator": "y3ll0w",
		"created_at": "Tue Aug 02 2022 10:57:16 GMT+0000 (Coordinated Universal Time)"
	}
}
```
Returns 401 on unauthorized request

---
DELETE **/{slug}**
> Delete the given slug

Header of Authorization with JWT

Returns 200 OK on success
Returns 401 on unauthorized request

---

GET **/{slug}**
> The redirct part

Returns 200 OK on success

---
POST **/user**
> Creates a new JWT for the user

Header of Authorization with the global admin key

```json
{
	"username":"y3ll0w"
}
```
Returns 200 OK on success with the body of 
```json
{
	"username": "y3ll0w",
	"token": "XXXXXXXXXXXXXXXX.XXXXXXXXXXXXXXXX.K9-XXXXXXXXXXXXXXXX"
}
```

Returns 401 on unauthorized request