use crate::controllers;
use crate::models::Db;
use warp::Filter;

pub fn all(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list_accounts(db.clone())
        .or(clear_stat(db.clone()))
        .or(index_stats(db.clone()))
        .or(update_stats(db.clone()))
        .or(add_logs_to_stats(db.clone()))
        .or(update_keyword_stats(db.clone()))
        .or(add_logs_to_keywords(db.clone()))
        .or(set_keywords_to_stats(db))
        .with(warp::trace::named("All Routes"))
}

pub fn clear_stat(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!(String / "clear_log")
        .and(warp::get())
        .and(with_db(db))
        .and_then(controllers::clear_log)
        .with(warp::trace::named("Route:Index Stats"))
}

pub fn list_accounts(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("list_accounts")
        .and(warp::get())
        .and(with_db(db))
        .and_then(controllers::list_accounts)
        .with(warp::trace::named("Route:Index Stats"))
}

pub fn index_stats(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!(String / "stats")
        .and(warp::get())
        .and(with_db(db))
        .and_then(controllers::stats)
        .with(warp::trace::named("Route:Index Stats"))
}

pub fn update_stats(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!(String / "stats")
        .and(warp::post())
        .and(warp::filters::body::json())
        .and(with_db(db))
        .and_then(controllers::update_stats)
        .with(warp::trace::named("Route: Update Stat"))
}

pub fn add_logs_to_stats(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!(String / "stats" / "add_logs")
        .and(warp::post())
        .and(warp::filters::body::json())
        .and(with_db(db))
        .and_then(controllers::add_logs_to_stats)
        .with(warp::trace::named("Route: Update Stat"))
}

pub fn set_keywords_to_stats(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!(String / "stats" / "set_keywords")
        .and(warp::post())
        .and(warp::filters::body::json())
        .and(with_db(db))
        .and_then(controllers::set_keywords_to_stats)
        .with(warp::trace::named("Route: Set Keyword to Stat"))
}

pub fn update_keyword_stats(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!(String / "update-keyword-stats")
        .and(warp::post())
        .and(warp::filters::body::json())
        .and(with_db(db))
        .and_then(controllers::update_keyword_stat)
        .with(warp::trace::named("Route: Update Keyword Statsitics "))
}

pub fn add_logs_to_keywords(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!(String / "stats" / u64 / "add_log")
        .and(warp::post())
        .and(warp::filters::body::json())
        .and(with_db(db))
        .and_then(controllers::add_logs_to_keyword)
        .with(warp::trace::named("Route: Add Log to Keywords "))
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
