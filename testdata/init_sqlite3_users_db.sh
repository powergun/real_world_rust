#!/usr/bin/env bash

rm -rf "users.db"
sqlite3 "users.db" <<"SQL"
create table users(u_name text PRIMARY KEY, p_word text);
create table transactions(
    u_from text, u_to text, t_date integer, t_amount text, 
    PRIMARY KEY (u_from, t_date), 
    FOREIGN KEY (u_from) REFERENCES users(u_name), 
    FOREIGN KEY (u_to) REFERENCES users(u_name) 
);
insert into users (u_name, p_word) values ("matt", "matt_pw"), ("dave", "dave_pw");
insert into transactions (u_from, u_to, t_date, t_amount) values ("dave", "matt", datetime("now"), 50);
SQL
