create table tags (
    id              serial8 primary key,
    robot_group_id  int8 not null references robot_groups (id),
    tag             text not null
);
