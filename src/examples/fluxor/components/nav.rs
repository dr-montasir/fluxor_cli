pub const DESKTOP_NAV: &str = r##"<nav class="header__nav">
                        <a href="https://docs.rs/fluxor/latest/fluxor" class="header__link" target="_blank">Docs</a>
                        <a href="https://crates.io/crates/fluxor_cli" class="header__link" target="_blank">Examples</a>
                        <a href="https://github.com/dr-montasir/fluxor" class="header__link" target="_blank">GitHub</a>
                        <a href="#get-started" class="header__link header__link--button">Get Started</a>
                    </nav>"##;

pub const MOBILE_NAV: &str = r##"<div class="header__mobile-nav" x-show="mobileMenu" x-cloak x-transition x-on:click.away="mobileMenu = false">
                    <a href="https://docs.rs/fluxor/latest/fluxor" class="header__link" target="_blank">Docs</a>
                    <a href="https://crates.io/crates/fluxor_cli" class="header__link" target="_blank">Examples</a>
                    <a href="https://github.com/dr-montasir/fluxor" class="header__link" target="_blank">GitHub</a>
                    <a href="#get-started" class="header__link header__link--button" style="text-align: center;">Get Started</a>
                </div>"##;