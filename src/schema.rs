table! {
    use diesel::sql_types::*;

    actors (actor_id) {
        actor_id -> Int4,
        url_slug -> Text,
        mononymous_name -> Nullable<Text>,
        given_name -> Nullable<Text>,
        surname -> Nullable<Text>,
        user_id -> Nullable<Int4>,
        is_latest -> Bool,
        original_actor -> Int4,
        created_by -> Int4,
        created_date -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;

    event_actors (event_id, actor_id) {
        event_id -> Int4,
        actor_id -> Int4,
        perspective_id -> Nullable<Int4>,
        organization_id -> Nullable<Int4>,
    }
}

table! {
    use diesel::sql_types::*;

    event_dates (event_date_id) {
        event_date_id -> Int4,
        event_id -> Int4,
        perspective_id -> Nullable<Int4>,
        exact_date -> Nullable<Timestamp>,
        range_date_start -> Nullable<Timestamp>,
        range_date_end -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;

    event_locations (event_id, location_id) {
        event_id -> Int4,
        location_id -> Int4,
        perspective_id -> Nullable<Int4>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    event_sources (event_id, perspective_id) {
        event_id -> Int4,
        perspective_id -> Int4,
        url -> Nullable<Text>,
        description -> Nullable<Text>,
        source_type -> Nullable<Source_type_enum>,
    }
}

table! {
    use diesel::sql_types::*;

    events (event_id) {
        event_id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        url_slug -> Text,
        is_deleted -> Bool,
        is_latest -> Bool,
        original_event -> Int4,
        created_by -> Int4,
        created_date -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;

    locations (location_id) {
        location_id -> Int4,
        name -> Text,
    }
}

table! {
    use diesel::sql_types::*;

    merge_proposal_comments (merge_proposal_comment_id) {
        merge_proposal_comment_id -> Int4,
        comment -> Nullable<Text>,
        perspective_event_id -> Nullable<Int4>,
        perspective_event_property -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    merge_proposal_history (merge_proposal_history_id) {
        merge_proposal_history_id -> Int4,
        user_id -> Int4,
        action -> Merge_proposal_actionMapping,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    merge_proposals (merge_proposal_id) {
        merge_proposal_id -> Int4,
        source_perspective_id -> Int4,
        target_perspective_id -> Int4,
        status -> Merge_proposal_statusMapping,
        created_by -> Int4,
        created_date -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;

    organizations (organization_id) {
        organization_id -> Int4,
        name -> Text,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    perspective_events (perspective_event_id) {
        perspective_event_id -> Int4,
        event_id -> Int4,
        perspective_id -> Int4,
        name -> Nullable<Text>,
        description -> Nullable<Text>,
        is_deleted -> Bool,
        historicity_stance -> Historicity_stance_enumMapping,
        relevance_stance -> Int4,
        created_by -> Int4,
        created_date -> Timestamp,
        is_latest -> Bool,
        original_perspective_event -> Int4,
    }
}

table! {
    use diesel::sql_types::*;

    perspectives (perspective_id) {
        perspective_id -> Int4,
        url_slug -> Text,
        name -> Text,
        is_deleted -> Bool,
        is_latest -> Bool,
        original_perspective -> Int4,
        parent_perspective -> Nullable<Int4>,
        created_by -> Int4,
        created_date -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;

    topic_events (event_id, topic_id) {
        event_id -> Int4,
        topic_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;

    topics (topic_id) {
        topic_id -> Int4,
        name -> Text,
        url_slug -> Text,
        is_deleted -> Bool,
        original_topic -> Int4,
        created_by -> Int4,
        created_date -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    user_organizations (user_id, organization_id) {
        user_id -> Int4,
        organization_id -> Int4,
        role -> Nullable<User_organization_role_enum>,
    }
}

table! {
    use diesel::sql_types::*;

    users (user_id) {
        user_id -> Int4,
        username -> Text,
        email -> Text,
        password_hash -> Text,
        given_name -> Text,
        surname -> Nullable<Text>,
    }
}

joinable!(event_actors -> actors (actor_id));
joinable!(event_actors -> organizations (organization_id));
joinable!(event_locations -> locations (location_id));
joinable!(perspective_events -> users (created_by));
joinable!(perspective_events -> perspectives (perspective_id));
joinable!(perspective_events -> events (event_id));
joinable!(topic_events -> topics (topic_id));
joinable!(topic_events -> events (event_id));
joinable!(user_organizations -> organizations (organization_id));
joinable!(merge_proposal_comments -> perspective_events (perspective_event_id));

allow_tables_to_appear_in_same_query!(
    actors,
    event_actors,
    event_dates,
    event_locations,
    event_sources,
    events,
    locations,
    merge_proposal_comments,
    merge_proposal_history,
    merge_proposals,
    organizations,
    perspective_events,
    perspectives,
    topic_events,
    topics,
    user_organizations,
    users,
);
