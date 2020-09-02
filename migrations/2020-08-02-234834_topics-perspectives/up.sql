/* Create primary tables */

CREATE TABLE users (
  user_id SERIAL PRIMARY KEY,
  username TEXT UNIQUE NOT NULL,
  email TEXT NOT NULL,
  password_hash TEXT NOT NULL,
  given_name TEXT NOT NULL,
  surname TEXT
);

CREATE INDEX username_search ON users (username);

CREATE TABLE organizations (
  organization_id SERIAL PRIMARY KEY,
  name TEXT NOT NULL
);

CREATE TABLE topics (
  topic_id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  url_slug TEXT NOT NULL,
  is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
  original_topic INT NOT NULL DEFAULT lastVal(),
  created_by INT NOT NULL,
  created_date TIMESTAMP NOT NULL DEFAULT timezone('UTC'::text, NOW()),
  CONSTRAINT fk_original_topic FOREIGN KEY (original_topic) REFERENCES topics (topic_id),
  CONSTRAINT fk_created_by_user FOREIGN KEY (created_by) REFERENCES users (user_id)
);

CREATE INDEX topic_url_slug_search ON topics (url_slug);

CREATE TABLE events (
  event_id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT,
  url_slug TEXT NOT NULL,
  is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
  is_latest BOOLEAN NOT NULL DEFAULT TRUE,
  original_event INT NOT NULL DEFAULT lastVal(),
  created_by INT NOT NULL,
  created_date TIMESTAMP NOT NULL DEFAULT timezone('UTC'::text, NOW()),
  CONSTRAINT fk_original_event FOREIGN KEY (original_event) REFERENCES events (event_id),
  CONSTRAINT fk_created_by_user FOREIGN KEY (created_by) REFERENCES users (user_id)
);

CREATE INDEX event_url_slug_search ON events (url_slug);

CREATE TABLE perspectives (
  perspective_id SERIAL PRIMARY KEY,
  url_slug TEXT NOT NULL,
  name TEXT NOT NULL,
  is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
  is_latest BOOLEAN NOT NULL DEFAULT TRUE,
  original_perspective INT NOT NULL DEFAULT lastVal(),
  parent_perspective INT,
  created_by INT NOT NULL,
  created_date TIMESTAMP NOT NULL DEFAULT timezone('UTC'::text, NOW()),
  CONSTRAINT fk_parent_perspective FOREIGN KEY (parent_perspective) REFERENCES perspectives (perspective_id),
  CONSTRAINT fk_created_by_user FOREIGN KEY (created_by) REFERENCES users (user_id),
  CONSTRAINT fk_original_perspective FOREIGN KEY (original_perspective) REFERENCES perspectives (perspective_id)
);

CREATE INDEX perspective_url_slug_search ON perspectives (url_slug);

CREATE TABLE actors (
  actor_id SERIAL PRIMARY KEY,
  url_slug TEXT NOT NULL,
  mononymous_name TEXT,
  given_name TEXT,
  surname TEXT,
  user_id INT,
  is_latest BOOLEAN NOT NULL DEFAULT TRUE,
  original_actor INT NOT NULL DEFAULT lastVal(),
  created_by INT NOT NULL,
  created_date TIMESTAMP NOT NULL DEFAULT timezone('UTC'::text, NOW()),
  CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users(user_id),
  CONSTRAINT fk_created_by_user FOREIGN KEY (created_by) REFERENCES users (user_id),
  CONSTRAINT fk_original_actor FOREIGN KEY (original_actor) REFERENCES actors (actor_id)
);

CREATE INDEX actor_url_slug_search ON actors (url_slug);

CREATE TABLE locations (
  location_id SERIAL PRIMARY KEY,
  name TEXT NOT NULL
);

-- /* Create join tables */

CREATE TYPE user_organization_role_enum AS ENUM ('admin');

CREATE TABLE user_organizations (
  user_id INT NOT NULL,
  organization_id INT NOT NULL,
  role user_organization_role_enum,
  CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users (user_id),
  CONSTRAINT fk_organization_id FOREIGN KEY (organization_id) REFERENCES organizations (organization_id),
  PRIMARY KEY (user_id, organization_id)
);

CREATE TABLE topic_events (
  event_id INT NOT NULL,
  topic_id INT NOT NULL,
  CONSTRAINT fk_event FOREIGN KEY(event_id) REFERENCES events(event_id),
  CONSTRAINT fk_topic FOREIGN KEY(topic_id) REFERENCES topics(topic_id),
  PRIMARY KEY (event_id, topic_id)
);

CREATE TYPE historicity_stance_enum AS ENUM ('fact', 'fiction', 'unknown', 'leaning_fact', 'leaning_fiction');
CREATE DOMAIN relevance_stance_domain AS INT NOT NULL CHECK (value < 5 AND value >= 0);

CREATE TABLE perspective_events (
  perspective_event_id SERIAL PRIMARY KEY,
  event_id INT NOT NULL,
  perspective_id INT NOT NULL,
  name TEXT,
  description TEXT,
  is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
  historicity_stance historicity_stance_enum NOT NULL,
  relevance_stance relevance_stance_domain,
  created_by INT NOT NULL,
  created_date TIMESTAMP NOT NULL DEFAULT timezone('UTC'::text, NOW()),
  is_latest BOOLEAN NOT NULL DEFAULT TRUE,
  original_perspective_event INT NOT NULL DEFAULT lastVal(),
  CONSTRAINT fk_created_by FOREIGN KEY (created_by) REFERENCES users(user_id),
  CONSTRAINT fk_event FOREIGN KEY(event_id) REFERENCES events(event_id),
  CONSTRAINT fk_perspective FOREIGN KEY (perspective_id) REFERENCES perspectives(perspective_id),
  CONSTRAINT fk_original_perspective_event FOREIGN KEY (original_perspective_event) REFERENCES perspective_events(perspective_event_id)
);

CREATE TYPE source_type_enum AS ENUM ('primary', 'secondary', 'other');

CREATE TABLE event_sources (
  event_id INT NOT NULL,
  perspective_id INT,
  url TEXT,
  description TEXT,
  source_type source_type_enum,
  CONSTRAINT fk_event FOREIGN KEY (event_id) REFERENCES events (event_id),
  CONSTRAINT fk_perspective FOREIGN KEY (perspective_id) REFERENCES perspectives (perspective_id),
  PRIMARY KEY (event_id, perspective_id)
);

CREATE TABLE event_actors (
  event_id INT NOT NULL,
  actor_id INT NOT NULL,
  perspective_id INT,
  organization_id INT,
  CONSTRAINT fk_event FOREIGN KEY(event_id) REFERENCES events (event_id),
  CONSTRAINT fk_actor FOREIGN KEY(actor_id) REFERENCES actors (actor_id),
  CONSTRAINT fk_perspective FOREIGN KEY(perspective_id) REFERENCES perspectives(perspective_id),
  CONSTRAINT fk_organization FOREIGN KEY(organization_id) REFERENCES organizations(organization_id),
  PRIMARY KEY (event_id, actor_id)
);

CREATE TABLE event_locations (
  event_id INT NOT NULL,
  location_id INT NOT NULL,
  perspective_id INT,
  CONSTRAINT fk_event FOREIGN KEY (event_id) REFERENCES events(event_id),
  CONSTRAINT fk_location FOREIGN KEY (location_id) REFERENCES locations(location_id),
  CONSTRAINT fk_perspective FOREIGN KEY(perspective_id) REFERENCES perspectives(perspective_id),
  PRIMARY KEY (event_id, location_id)
);

CREATE TABLE event_dates (
  event_date_id SERIAL PRIMARY KEY,
  event_id INT NOT NULL,
  perspective_id INT,
  exact_date TIMESTAMP,
  range_date_start TIMESTAMP,
  range_date_end TIMESTAMP,
  CONSTRAINT fk_event FOREIGN KEY(event_id) REFERENCES events(event_id),
  CONSTRAINT fk_perspective FOREIGN KEY(perspective_id) REFERENCES perspectives(perspective_id)
);

/* Initial seed data */
do $$
DECLARE
  local_user INT;
  mam_topic INT;
  conviction_event_id INT;
  murder_event_id INT;
  steven_avery_actor_id INT;
  courthouse_location_id INT;
  avery_property_location_id INT;
  quarry_location_id INT;
  consensus_perspective_id INT;
  prosecution_perspective_id INT;
  defense_perspective_id INT;
BEGIN
  INSERT INTO users (username, email, password_hash, given_name, surname) VALUES
    ('localadmin', 'local@example.com', '$2b$10$LTPQUUMMhdPyEcjk4CB8tewuGC48DaLh8AfbiZJQ1gW0U8yCq0jLy', 'Local', 'User') RETURNING user_id INTO local_user
  ;

  INSERT INTO topics (name, url_slug, created_by) VALUES
    ('Death of Teresa Halbach', 'Death-Of-Teresa-Halbach', local_user) RETURNING topic_id INTO mam_topic
  ;

  INSERT INTO events (name, description, url_slug, created_by) VALUES ('Steven Avery is convicted of the murder of Teresa Halbach', NULL, 'steven_avery_convicted', local_user) RETURNING event_id INTO conviction_event_id;
  INSERT INTO events (name, description, url_slug, created_by) VALUES ('Teresa Halbach is murdered', NULL, 'Teresa-Halbach-murdered', local_user) RETURNING event_id INTO murder_event_id;

  INSERT INTO actors (url_slug, given_name, surname, created_by) VALUES ('Steven_Avery', 'Steven', 'Avery', local_user) RETURNING actor_id INTO steven_avery_actor_id;

  INSERT INTO locations (name) VALUES ('Manitowoc County Courthouse') RETURNING location_id INTO courthouse_location_id;
  INSERT INTO locations (name) VALUES ('Avery Property') RETURNING location_id INTO avery_property_location_id;
  INSERT INTO locations (name) VALUES ('Michels Materials quarry') RETURNING location_id INTO quarry_location_id;

  INSERT INTO perspectives (url_slug, name, created_by) VALUES ('Death-Of-Teresa-Halbach-Consensus', 'Consensus', local_user) RETURNING perspective_id INTO consensus_perspective_id;
  INSERT INTO perspectives (url_slug, name, created_by, parent_perspective) VALUES ('Original-Steven-Avery-Prosecution', 'Original Steven Avery Prosecution', local_user, consensus_perspective_id) RETURNING perspective_id INTO prosecution_perspective_id;
  INSERT INTO perspectives (url_slug, name, created_by, parent_perspective) VALUES ('Original-Steven-Avery-Defense', 'Original Steven Avery Defense', local_user, consensus_perspective_id) RETURNING perspective_id INTO defense_perspective_id;

  INSERT INTO topic_events (topic_id, event_id) VALUES
    (mam_topic, conviction_event_id),
    (mam_topic, murder_event_id)
  ;

  INSERT INTO perspective_events (event_id, perspective_id, name, historicity_stance, relevance_stance, created_by) VALUES
    (murder_event_id, prosecution_perspective_id, 'Steven Avery and Brandon Dassey murder Teresa Halbach', 'fact', 0, local_user),
    (murder_event_id, defense_perspective_id, 'Teresa Halbach was murdered, but not by Steven Avery', 'fact', 0, local_user)
  ;

  INSERT INTO event_sources (event_id, perspective_id, url, source_type) VALUES
    (conviction_event_id, consensus_perspective_id, 'https://web.archive.org/web/20160118124033/http://www.jsonline.com/news/wisconsin/29388834.html', 'secondary')
  ;

  INSERT INTO event_actors (event_id, actor_id, perspective_id) VALUES
  (conviction_event_id, steven_avery_actor_id, NULL),
  (murder_event_id, steven_avery_actor_id, prosecution_perspective_id)
  ;

  INSERT INTO event_locations (event_id, location_id, perspective_id) VALUES
    (conviction_event_id, courthouse_location_id, consensus_perspective_id),
    (murder_event_id, avery_property_location_id, prosecution_perspective_id)
  ;

  INSERT INTO event_dates (event_id, perspective_id, exact_date, range_date_start, range_date_end) VALUES
    (conviction_event_id, consensus_perspective_id, '2007-03-19 12:00:00', NULL, NULL),
    (murder_event_id, prosecution_perspective_id, '2005-11-01 12:00:00', NULL, NULL),
    (murder_event_id, defense_perspective_id, NULL, '2005-10-31 12:00:00', '2005-11-04 12:00:00')
  ;
END $$;
