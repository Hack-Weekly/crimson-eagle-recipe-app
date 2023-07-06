# Crimson Eagle - Recipe Site

## User Authentication Endpoints

### POST /register - Registers a new user

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
	"username": "John Doe",
	"password": "123456"
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
	"username": "John Doe",
	"password": "123456"
}
```

Response:

```json
{
	"id": 1234,
	"username": "John Doe"
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
	"username": "John Doe"
}
```
