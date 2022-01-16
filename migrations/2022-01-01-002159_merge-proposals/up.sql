CREATE TYPE merge_proposal_status AS ENUM ('draft', 'open', 'closed', 'rejected', 'approved', 'merged');

CREATE TABLE merge_proposals (
  merge_proposal_id SERIAL PRIMARY KEY,
  source_perspective_id INT NOT NULL,
  target_perspective_id INT NOT NULL,
  status merge_proposal_status NOT NULL,
  created_by INT NOT NULL,
  created_date TIMESTAMP NOT NULL DEFAULT timezone('UTC'::text, NOW()),
  CONSTRAINT fk_source_perspective FOREIGN KEY (source_perspective_id) REFERENCES perspectives (perspective_id),
  CONSTRAINT fk_target_perspective FOREIGN KEY (target_perspective_id) REFERENCES perspectives (perspective_id),
  CONSTRAINT fk_created_by FOREIGN KEY (created_by) REFERENCES users (user_id),
  UNIQUE (source_perspective_id, target_perspective_id)
);

CREATE TYPE merge_proposal_action AS ENUM ('createdraft', 'create', 'comment', 'close', 'approve', 'reject', 'merge');

CREATE TABLE merge_proposal_history (
  merge_proposal_history_id SERIAL PRIMARY KEY,
  merge_proposal_id INT NOT NULL,
  user_id INT NOT NULL,
  action merge_proposal_action NOT NULL,
  CONSTRAINT fk_merge_proposal_id FOREIGN KEY (merge_proposal_id) REFERENCES merge_proposals (merge_proposal_id),
  CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users (user_id)
);

CREATE TABLE merge_proposal_comments (
  merge_proposal_comment_id SERIAL PRIMARY KEY,
  merge_proposal_history_id INT NOT NULL,
  comment TEXT,
  perspective_event_id INT,
  perspective_event_property TEXT,
  CONSTRAINT fk_perspective_event_id FOREIGN KEY (perspective_event_id) REFERENCES perspective_events (perspective_event_id),
  CONSTRAINT fk_merge_proposal_history_id FOREIGN KEY (merge_proposal_history_id) REFERENCES merge_proposal_history (merge_proposal_history_id)
);