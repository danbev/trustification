use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew_more_hooks::prelude::*;
use yew_nested_router::prelude::Switch as RouterSwitch;

use crate::pages::View;
use crate::{
    about,
    backend::Endpoint,
    hooks::use_backend::use_backend,
    pages::{self, AppRoute},
};

#[function_component(Console)]
pub fn console() -> Html {
    let brand = html! (
        <MastheadBrand>
            <Brand
                src="assets/images/chicken-svgrepo-com.svg"
                alt="Logo"
                style={r#"
                    --pf-v5-c-brand--Height: var(--pf-v5-c-page__header-brand-link--c-brand--MaxHeight);
                "#}
            >
                <BrandSource srcset="assets/images/chicken-svgrepo-com.svg" />
            </Brand>
        </MastheadBrand>
    );

    let backend = use_backend();

    let sidebar = html_nested!(
        <PageSidebar>
            <Nav>
                <NavList>
                    <NavRouterItem<AppRoute> to={AppRoute::Index}>{ "Trusted Content" }</NavRouterItem<AppRoute>>
                    <NavExpandable title="Search">
                        <NavRouterItem<AppRoute> to={AppRoute::Package(Default::default())} predicate={AppRoute::is_package}>{ "Packages" }</NavRouterItem<AppRoute>>
                        <NavRouterItem<AppRoute> to={AppRoute::Advisory(Default::default())} predicate={AppRoute::is_advisory}>{ "Advisories" }</NavRouterItem<AppRoute>>
                    </NavExpandable>
                    <NavExpandable title="Extend">
                        if let Ok(url) = backend.join(Endpoint::Api, "/swagger-ui/") {
                            <NavItem external=true target="_blank" to={url.to_string()}>{ "API" }</NavItem>
                        }
                        if let Ok(url) = backend.join(Endpoint::Bombastic, "/swagger-ui/") {
                            <NavItem external=true target="_blank" to={url.to_string()}>{ "SBOM API" }</NavItem>
                        }
                        if let Ok(url) = backend.join(Endpoint::Vexination, "/swagger-ui/") {
                            <NavItem external=true target="_blank" to={url.to_string()}>{ "VEX API" }</NavItem>
                        }
                    </NavExpandable>
                </NavList>
            </Nav>
        </PageSidebar>
    );

    let callback_github = use_open("https://github.com/trustification/trustification", "_blank");

    let backdrop = use_backdrop();

    let callback_about = Callback::from(move |_| {
        if let Some(backdrop) = &backdrop {
            backdrop.open(html!(<about::About/>));
        }
    });

    let tools = html!(
        <Toolbar>
            <ToolbarContent>
                <ToolbarItem modifiers={[ToolbarElementModifier::Right]}>
                    <Button icon={Icon::Github} onclick={callback_github} variant={ButtonVariant::Plain} />
                    <Dropdown
                        position={Position::Right}
                        variant={MenuToggleVariant::Plain}
                        icon={Icon::QuestionCircle}
                    >
                        <MenuAction text="About" onclick={callback_about} />
                    </Dropdown>
                </ToolbarItem>
            </ToolbarContent>
        </Toolbar>
    );

    html!(
        <Page {brand} {sidebar} {tools}>
            <RouterSwitch<AppRoute> {render}/>
        </Page>
    )
}

fn render(route: AppRoute) -> Html {
    match route {
        AppRoute::Index => html!(<pages::Index/>),
        AppRoute::Chicken => html!(<pages::Chicken/>),
        AppRoute::Package(View::Search { query }) => html!(<pages::Package {query} />),
        AppRoute::Package(View::Content { id }) => html!(<pages::SBOM {id} />),
        AppRoute::Advisory(View::Search { query }) => html!(<pages::Advisory {query} />),
        AppRoute::Advisory(View::Content { id }) => html!(<pages::VEX {id} />),
    }
}
