# Restaurant Service & Client

## Instructions

1. Run the server with `cargo run --bin server` 
2. After server has been initialized, run the client with `cargo run --bin client`
3. Client UI can now be used to send various kinds of requests to the server.

## Shortcuts:

1. Very minimal logging setup.
2. The service currently uses an in-memory sqlite database instead of an actual database. Therefore each time the server is closed and started, the entire DB is reset back to zero. In a real use case, this should be an actual database to persist data across sessions.
3. There are a lot of `unwraps` in the code, mostly because we are working with a static data set. In a real use case, it would be better to error out and notify the client, log the issue, notify webhooks, etc.
4. Arguably it's more correct if instead of the orders being tied to tables, they would be tied to a party (which sits at a table). Then other useful functions like being able to check-out an entire party once the customers leave and clear out all of the ordered items without having to do it one by one.
5. `table_id` in this case is just a raw integer. There should probably be some kind of "Restaurant Configuration" which stored how many tables exist, as well as what the identifiers should be. A restaurant owner may want to label specific tables special names to help describe them, like "Booth A," "Patio 2," or "Bar." This would then require a 3rd table for the table configuration, and subsequent foreign key setups as needed.
6. I'm also using autoincrement integer primary keys all over the place. This is bad for a number of reasons (we can discuss this in more detail), and its also not a good idea to be "leaking" these ids to client unless absolutely necessary. Although in this case, since we are responsible for both server and client, it's not so bad.
7. I would normally use newtypes for IDs to keep things clear and consistent. For this case, just in the interest of time I've skipped this.