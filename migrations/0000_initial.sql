create table items (
    item_id integer not null primary key,
    name text not null unique
);

create table lists (
    list_id integer not null primary key,
    name text not null unique,
    created timestamp not null,
    finished timestamp
);

create table list_items (
    list_id integer not null references lists(list_id),
    item_id integer not null references items(item_id),
    amount integer not null,
    primary key (list_id, item_id)
);
