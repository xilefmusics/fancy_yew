use super::NavItemBuilder;

pub trait Navable
where
    Self: Sized,
{
    fn route_items() -> Vec<Self>;
    fn to_nav_item(self) -> NavItemBuilder<'static>;
}
