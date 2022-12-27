use anyhow::Result;
use axum::{
    response::{Html, Redirect},
    routing::get,
    Router, Server,
};
use leptos::*;

async fn handle_error(err: std::io::Error) -> (http::StatusCode, String) {
    (
        http::StatusCode::NOT_FOUND,
        format!("File not found: {err}"),
    )
}

#[component]
fn Footer(cx: Scope) -> impl IntoView {
    view! { cx,
        <footer id="footer" class="text-black dark:text-gray-100">
            <div class="container flex flex-col-reverse justify-between px-6 py-10 mx-auto space-y-8 md:space-y-0">
                <div class="text-xs block text-center">
                    "Copyright © 2023, All rights reserved<br>Made by AOx0 with Leptos SSR &amp; TailwindCSS"
                </div>
            </div>
        </footer>
    }
}

#[component]
fn MoonIcon(cx: Scope) -> impl IntoView {
    view! {cx,
        <svg class="hidden dark:block" height="1em" viewBox="0 0 50 50" width="1em" xmlns="http://www.w3.org/2000/svg">
            <svg::path d="M 43.81 29.354 C 43.688 28.958 43.413 28.626 43.046 28.432 C 42.679 28.238 42.251 28.198 41.854 28.321 C 36.161 29.886 30.067 28.272 25.894 24.096 C 21.722 19.92 20.113 13.824 21.683 8.133 C 21.848 7.582 21.697 6.985 21.29 6.578 C 20.884 6.172 20.287 6.022 19.736 6.187 C 10.659 8.728 4.691 17.389 5.55 26.776 C 6.408 36.163 13.847 43.598 23.235 44.451 C 32.622 45.304 41.28 39.332 43.816 30.253 C 43.902 29.96 43.9 29.647 43.81 29.354 Z" fill="currentColor"/>
        </svg>
    }
}

#[component]
fn SunIcon(cx: Scope) -> impl IntoView {
    view! { cx,
        <svg class="block dark:hidden" width="1em" viewBox="0 0 24 24" height="1em" fill="none" xmlns="http://www.w3.org/2000/svg">
            <svg::circle r="5.75375" fill="currentColor" cx="11.9998" cy="11.9998"/>
            <svg::g>
                <svg::circle transform="rotate(-60 3.08982 6.85502)" fill="currentColor" cx="3.08982" cy="6.85502" r="1.71143"/>
                <svg::circle r="1.71143" cx="3.0903" cy="17.1436" transform="rotate(-120 3.0903 17.1436)" fill="currentColor"/>
                <svg::circle r="1.71143" cx="12" cy="22.2881" fill="currentColor"/>
                <svg::circle transform="rotate(-60 20.9101 17.1436)" cy="17.1436" cx="20.9101" r="1.71143" fill="currentColor"/>
                <svg::circle cy="6.8555" r="1.71143" fill="currentColor" cx="20.9101" transform="rotate(-120 20.9101 6.8555)"/>
                <svg::circle fill="currentColor" cy="1.71143" r="1.71143" cx="12"/>
            </svg::g>

        </svg>
    }
}

#[component]
pub fn MenuItem(
    cx: Scope,
    href: &'static str,
    #[prop(optional)] more: &'static str,
    #[prop(optional)] nopage: bool,
    #[prop(optional)] color: &'static str,
    children: Box<dyn Fn(Scope) -> Fragment>,
) -> impl IntoView {
    let color = if color == "" {
        "text-orange-500"
    } else {
        color
    };
    view! {cx,
        {if nopage {
            view!{cx,
                <a
                    target="_blank"
                    rel="noopener noreferrer"
                    class={&format!("hover:{} {}", color, more)}
                    href=href
                >
                    {children(cx)}
                </a>
            }
        } else {
            view!{cx,
                <a
                    class={&format!("hover:{} {}", color, more)}
                    href=href
                >
                    {children(cx)}
                </a>
            }
        }}
    }
}

#[component]
fn LinkedIn(cx: Scope) -> impl IntoView {
    view! { cx,
        <svg
            view_box="0 0 24 24" fill="currentColor" width="24" height="24"
        >
            <svg::path
                d="M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z"
            />
        </svg>
    }
}

#[component]
fn Github(cx: Scope) -> impl IntoView {
    view! { cx,
        <svg fill="currentColor" viewBox="0 0 16 16" width="24" height="24">
            <svg::path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z" fill-rule="evenodd"/>
        </svg>
    }
}

#[component]
pub fn ItemsCollection(cx: Scope) -> impl IntoView {
    view! { cx,
        <MenuItem href="/home">"Home"</MenuItem>
        // <MenuItem href="/portfolio">"Portfolio"</MenuItem>
        <MenuItem href="/blog">"Blog"</MenuItem>
        // <MenuItem href="/resume">"Resume"</MenuItem>
        <MenuItem href="/contact">"Contact"</MenuItem>
    }
}

#[component]
pub fn IconsCollection(cx: Scope) -> impl IntoView {
    view! { cx,
        <MenuItem
            nopage=true
            href=r"https://www.linkedin.com/in/aox0/"
        >
            <LinkedIn/>
        </MenuItem>
        <MenuItem
            nopage=true
            href=r"https://github.com/AOx0"
        >
            <Github/>
        </MenuItem>
    }
}

#[component]
pub fn AOx0(cx: Scope) -> impl IntoView {
    view! { cx,
        <MenuItem href="/">
            <h1 class="text-4xl font-bold">
                "AOx0"
            </h1>
        </MenuItem>
    }
}

#[component]
fn Menu(cx: Scope) -> impl IntoView {
    view! { cx,
            <ItemsCollection/>
            <IconsCollection/>
            <button class="pt-1 hover:text-orange-500"
                onclick="
                    const html = document.getElementsByTagName('html')[0];
                    if(html.classList.contains('dark')) {
                        document.getElementById('t_color').content = 'black'
                        html.classList.remove('dark');
                        localStorage.theme = 'light'
                    } else {
                        document.getElementById('t_color').content = 'rgb(31 41 55 / var(--tw-bg-opacity))'
                        html.classList.add('dark');
                        localStorage.theme = 'dark'
                    }
                "
            >
                <MoonIcon/>
                <SunIcon/>
            </button>
    }
}

#[component]
fn NavBar(cx: Scope) -> impl IntoView {
    view! {cx,
        <nav class="relative container v-screen mx-auto pt-6 md:py-6 px-10  text-black dark:text-gray-100">
            <div class="flex items-center justify-between">
                <div class="pt-2">
                    <AOx0/>
                </div>
                <div class="hidden md:flex space-x-6">
                    <Menu/>
                </div>
            </div>
            <div class="flex flex-wrap md:hidden justify-center space-x-5 md:space-x-6 space-y-2 container v-screen mx-auto py-6 px-10 text-black dark:text-gray-100">
                <p>" "</p>
                <Menu/>
            </div>
        </nav>
    }
}

#[component]
fn BaseHtml(
    cx: Scope,
    #[prop(optional)] x_data: &'static str,
    #[prop(optional)] katex: bool,
    #[prop(optional)] alpine: bool,
    children: Box<dyn Fn(Scope) -> Fragment>,
) -> impl IntoView {
    view! { cx,
        <html class="dark">
            <head>
                <meta id="t_color" name="theme-color" content="rgb(31 41 55 / var(--tw-bg-opacity))"/>
                <meta charset="UTF-8"/>
                <link rel="stylesheet" href="/static/styles.css"/>
                {(alpine || x_data != "").then(|| view!{cx, <script src=r"https://unpkg.com/alpinejs@3.x.x/dist/cdn.min.js" defer init />})}
                <script>
                    "const html = document.getElementsByTagName('html')[0];
                    if (localStorage.theme === 'dark' || !('theme' in localStorage)) {
                        document.getElementById('t_color').content = 'rgb(31 41 55 / var(--tw-bg-opacity))'
                        html.classList.add('dark');
                        localStorage.theme = 'dark'
                    } else {
                        document.getElementById('t_color').content = 'black'
                        html.classList.remove('dark');
                        localStorage.theme = 'light'
                    }"
                </script>
                {katex.then(||
                    view! { cx,
                        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.0/dist/katex.min.css" integrity="sha384-Xi8rHCmBmhbuyyhbI88391ZKP2dmfnOl4rT9ZfRI7mLTdk1wblIUnrIq35nqwEvC" crossorigin="anonymous"/>
                        <script defer src=r"https://cdn.jsdelivr.net/npm/katex@0.16.0/dist/katex.min.js" integrity="sha384-X/XCfMm41VSsqRNQgDerQczD69XqmjOOOwYQvr/uuC+j4OPoNhVgjdGFwhvN02Ja" crossorigin="anonymous"></script>
                        <script defer src=r"https://cdn.jsdelivr.net/npm/katex@0.16.0/dist/contrib/auto-render.min.js" integrity="sha384-+XBljXPPiv+OzfbB3cVmLHf4hdUFHlWNZN5spNQ7rmHTXpd7WvJum6fIACpNNfIR" crossorigin="anonymous"></script>
                        <script>
                            "document.addEventListener('DOMContentLoaded', function() {
                                renderMathInElement(document.body, {
                                    delimiters: [
                                        {left: '$$', right: '$$', display: true},
                                        {left: '$', right: '$', display: false}
                                    ],
                                    throwOnError : false
                                });
                            });"
                        </script>
                    }
                )}
            </head>
            <NavBar/>
            <body
                x-data=x_data
                class="flex flex-col h-screen bg-white dark:bg-gray-800 text-black dark:text-gray-100"
            >
            {children(cx)}
            </body>
            <Footer/>
        </html>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <BaseHtml alpine=true>
            <></>
        </BaseHtml>
    }
}

async fn say_hello() -> Html<String> {
    Html(render_to_string(|cx| view! {cx, <Home /> }))
}

#[tokio::main]
async fn main() -> Result<()> {
    let static_service = axum::error_handling::HandleError::new(
        tower_http::services::ServeDir::new("./static"),
        handle_error,
    );

    let app = Router::new()
        .route(
            "/favicon.ico",
            get(|| async { Redirect::permanent("/static/favicon.ico") }),
        )
        .route("/", get(say_hello))
        .nest_service("/static/", static_service);

    Ok(Server::bind(&"0.0.0.0:8000".parse()?)
        .serve(app.into_make_service())
        .await?)
}
