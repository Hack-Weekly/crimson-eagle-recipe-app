# Crimson Eagle - Recipe Site

## User Authentication Endpoints

### POST /register - Registers a new user

Registers a new user with the provided username and password. 

##### Needs:
- The username must be at least 3 characters long and can only contain letters and numbers. 
- The password must be at least 6 characters long, contain at least one letter and one number, and can optionally include certain special characters.

Headers:

```json
{
	"Content-Type": "application/json"
}
```

Body:

```json
{
	"username": "JohnDoe",
	"password": "Password123456"
}
```

### POST /login - Signs a user in

Needs:

Headers:

```json
{
	"Content-Type": "application/json"
}
```

Body:

```json
{
	"username": "JohnDoe",
	"password": "Password123456"
}
```

Response:

```json
{
	"id": 1234,
	"username": "JohnDoe"
}
```

### GET /profile - Returns the logged in user's data

Needs:

Headers:

```json
{
	"Content-Type": "application/json",
	"Authentication": "Bearer {token received from login}"
}
```

Response:

```json
{
	"username": "JohnDoe"
}
```
