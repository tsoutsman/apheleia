mod archetype;
mod item;
mod loan;
mod role;
mod settings;
mod subject_area;
mod user;

pub(crate) fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.configure(archetype::config)
        .configure(item::config)
        .configure(loan::config)
        .configure(role::config)
        .configure(settings::config)
        .configure(subject_area::config)
        .configure(user::config);
}
