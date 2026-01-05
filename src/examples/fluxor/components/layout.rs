use fluxor::cans::content::do_html;
use fluxor::wtime;

use crate::components::*;

pub const LAYOUT_TEMPLATE: &str = r###"<!DOCTYPE html>
<html lang="en">
    <!-- head -->
    {{HEAD}}
    <body>
        <!-- main container -->
        <div x-data="{ mobileMenu: false }">
            <!-- header -->
            {{HEADER}}

            <!-- main content -->
            {{MAIN_CONTENT}}

            <!-- footer -->
            {{FOOTER}}
        </div>

        <!-- scripts -->
        <!-- service worker register -->
        {{SW_REGISTER_SCRIPT}}
    </body>
</html>"###;

pub fn layout(title: &str, description: &str, keywords: &str, main_content: &str) -> String {
    let year = wtime::local::get_local_year();

    do_html!(
        LAYOUT_TEMPLATE,
        HEAD = head(title, description, keywords),
        HEADER = header(),
        MAIN_CONTENT=main_content,
        FOOTER=footer(year),
        SW_REGISTER_SCRIPT = SW_REGISTER
    )
}