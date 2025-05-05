# Rust API generator

We use rust macros to parse basic DDL statements and generate  api endpoints for Actix at compile time. 

For example with the following ddl statement:

```
CREATE TABLE ITEMS(ID INT, name text, description text)
```

We would generate at compile time the following endpoints, along with the appropriate SQL and type safe structs


1. **GET /items**
   - **Description**: Retrieve a list of all items.
   - **Response**:
     - **200 OK**: Returns a JSON array of items.
     - **Example**:
       ```json
       [
         {
           "id": 1,
           "name": "Item 1",
           "description": "Description for Item 1"
         },
         {
           "id": 2,
           "name": "Item 2",
           "description": "Description for Item 2"
         }
       ]
       ```

2. **POST /items**
   - **Description**: Create a new item.
   - **Request Body**:
     - **Content-Type**: `application/json`
     - **Example**:
       ```json
       {
         "name": "NewItem",
         "description": "Description for NewItem"
       }
       ```
   - **Response**:
     - **201 Created**: Returns the created item.
     - **Example**:
       ```json
       {
         "id": 3,
         "name": "NewItem",
         "description": "Description for NewItem"
       }
       ```

3. **PATCH /items/{id}**
   - **Description**: Update an existing item.
   - **Path Parameters**:
     - `id`: The ID of the item to update.
   - **Request Body**:
     - **Content-Type**: `application/json`
     - **Example**:
       ```json
       {
         "name": "UpdatedItem",
         "description": "Updated description for Item"
       }
       ```
   - **Response**:
     - **200 OK**: Returns the updated item.
     - **Example**:
       ```json
       {
         "id": 1,
         "name": "UpdatedItem",
         "description": "Updated description for Item 1"
       }
       ```

4. **DELETE /items/{id}**
   - **Description**: Delete an existing item.
   - **Path Parameters**:
     - `id`: The ID of the item to delete.
   - **Response**:
     - **204 No Content**: Indicates that the item was successfully deleted.

### Example Usage

- **GET /items**: Retrieves all items in the collection.
- **POST /items**: Creates a new item with the provided name and description.
- **PATCH /items/1**: Updates the item with ID 1.
- **DELETE /items/1**: Deletes the item with ID 1.


# Generating a useable schema:

`pg_dump -d $DATABASE_URL -s | sed '/^--/d' >schema.sql`


# Running tests

If the outuput is not what you expect, you can run tests for the parser and macros. The macro will ignore any input that it cannot parse and output a warning. 

`cd parser`

`cargo test -- --nocapture`

Will dump the parsed contents of the schema.sql file for manual examination
