# Crimson Eagle - Recipe Site

# Phase 1 Plan

This plan will outline requirements for the front-end and back-end team to acheive a working minimum viable product (MVP). Once these requirements have been met we will decide on extra features, either by poll or suggestions from the team.

## Front End


Build a single web page that displays information from the API provided by the Back-end team. This webpage should display recipes in a layout that suits the information such as a grid or table. There should be a button to delete the recipe somewhere on the page.

The webpage should also include a form to add information to the API. This should be either linked with a button from the main page or displayed on the same page as the recipes.

Decisions about styling and layout are completely up to the front-end team. The project is being completed using React in the NextJS framework. CSS is up to the front-end team to decide. 

The API endpoints that will be created will be displayed in the next section in JSON format for sending API requests.

## Back End

*Note : API URL will be provided upon deployment*

Create an API in Rust with the following endpoints:

All Requests can be sent with the following header:

```json Headers
{
	"Content-Type": "application/json",
}
```

GET /recipes - to display recipes on the screen

```json
{
  "records": [
    {
        "id": 1234,
        "title": "Recipe title",
        "servings": "4",
        "ingredients": [
            {
            "amount": "1 cup",
            "ingredient": "milk",
            },
            {
            "amount": "1 tsp",
            "ingredient": "sugar",
            }
        ],
        "instructions": [
            "inst1",
            "inst2"
        ]
    },
    {
        "id": 1235,
        ...
    }
  ]
}

```
POST /recipes - to add a recipe

```json
{
  "title": "Recipe title",
  "servings": "4",
  "ingredients": [
    {
      "amount": "1 cup",
      "ingredient": "milk",
    },
    {
      "amount": "1 tsp",
      "ingredient": "sugar",
    }
  ],
  "instructions": [
    "inst1",
    "inst2"
  ]
}
```
DELETE /recipes/\<id>

```json
{
    "id" : id,
}
```

PUT /recipes/\<id> - to edit a recipe

```json
 {
        "id": id,
        "title": "Updated title",
        "servings": "Updated servings",
        "ingredients": [
            {
            "amount": "updated",
            "ingredient": "updated",
            },
            {
            "amount": "updated",
            "ingredient": "updated",
            }
        ],
        "instructions": [
            "updated inst1",
            "updated inst2"
        ]
    },
```


Store the recipe data on a database.

## Other Notes

Please speak up if you have any issues. There are experienced people in the group who can answer questions and suggest resources to read. 

Most importantly - Have fun coding!


