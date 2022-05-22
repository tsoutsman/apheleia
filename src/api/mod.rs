// TODO: Make pub(crate) only for cfg(test)

pub(crate) mod archetype;
pub(crate) mod item;
pub(crate) mod loan;
pub(crate) mod role;
pub(crate) mod settings;
pub(crate) mod subject_area;
pub(crate) mod user;

pub(crate) fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.configure(archetype::config)
        .configure(item::config)
        .configure(loan::config)
        .configure(role::config)
        .configure(settings::config)
        .configure(subject_area::config)
        .configure(user::config);
}
