use std::{env, net::IpAddr};

use actix_cors::Cors;
use actix_web::{
    http::{header, Uri},
    middleware::Logger,
    App, HttpServer,
};
use dotenvy::dotenv;
use rest::main::{
    menu::{
        add_meal_to_menu_endpoint::add_meal_to_menu_endpoint_config,
        get_health_status::get_health_status_config,
        get_meal_by_id_endpoint::get_meal_by_id_endpoint_config,
        get_menu_endpoint::get_menu_endpoint_config,
        remove_meal_from_menu_endpoint::remove_meal_from_menu_endpoint_config,
    },
    order::{
        cancel_order_endpoint::cancel_order_endpoint_config,
        confirm_order_endpoint::confirm_order_endpoint_config,
        get_order_by_id_endpoint::get_order_by_id_endpoint_config,
        get_orders_endpoint::get_orders_endpoint_config,
    },
};

use crate::main::configuration::use_case_configuration::{
    ADD_MEAL_TO_MENU_USE_CASE, CANCEL_ORDER_USECASE, CONFIRM_ORDER_USECASE,
    GET_MEAL_BY_ID_USE_CASE, GET_MENU_USE_CASE, GET_ORDERS_USECASE, GET_ORDER_BY_ID,
    REMOVE_MEAL_FROM_MENU_USECASE,
};

#[actix_web::main]
pub async fn start_web_backend() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(
        env_logger::Env::new().default_filter_or(env::var("LOG_LEVEL").unwrap()),
    );
    log::info!("Log level is set to {:?}", env::var("LOG_LEVEL").unwrap());

    let host_url = env::var("HOST_URL").unwrap().parse::<Uri>().unwrap();
    let host_address = host_url.host().unwrap();
    let host_port = host_url.port().unwrap();

    log::info!("starting HTTP server at {}", env::var("HOST_URL").unwrap());

    HttpServer::new(move || {
        App::new()
            .configure(get_health_status_config)
            .configure(add_meal_to_menu_endpoint_config)
            .configure(get_meal_by_id_endpoint_config)
            .configure(get_menu_endpoint_config)
            .configure(remove_meal_from_menu_endpoint_config)
            .configure(cancel_order_endpoint_config)
            .configure(confirm_order_endpoint_config)
            .configure(get_order_by_id_endpoint_config)
            .configure(get_orders_endpoint_config)
            .app_data(ADD_MEAL_TO_MENU_USE_CASE.clone())
            .app_data(GET_MEAL_BY_ID_USE_CASE.clone())
            .app_data(GET_MENU_USE_CASE.clone())
            .app_data(REMOVE_MEAL_FROM_MENU_USECASE.clone())
            .app_data(CANCEL_ORDER_USECASE.clone())
            .app_data(CONFIRM_ORDER_USECASE.clone())
            .app_data(GET_ORDER_BY_ID.clone())
            .app_data(GET_ORDERS_USECASE.clone())
            .wrap(
                Cors::default()
                    .allowed_origin(&env::var("HOST_URL").unwrap())
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![
                        header::AUTHORIZATION,
                        header::ACCEPT,
                        header::LOCATION,
                        header::CONTENT_TYPE,
                    ])
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(Logger::default())
    })
    .bind((
        host_address
            .parse::<IpAddr>()
            .expect("Wrong IP address configured"),
        host_port
            .to_string()
            .parse::<u16>()
            .expect("Wrong port configured"),
    ))?
    .run()
    .await
}
