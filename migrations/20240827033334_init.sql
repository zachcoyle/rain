create table if not exists Location (
  id integer primary key,
  geohash text not null,
  name text not null
);

create table Meteo (
  id integer primary key,
  location_id integer not null,
  response text not null,
  timestamp text not null,
  foreign key (location_id) --
  references Location (id) -- 
  on delete cascade --
  on update no action
)
