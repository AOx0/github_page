#![allow(dead_code)]

use anyhow::Result;
use axum::extract::Path;
use axum::{
    response::{Html, Redirect},
    routing::get,
    Router, Server,
};
use comrak::plugins::syntect::SyntectAdapter;
use comrak::{markdown_to_html_with_plugins, ComrakOptions, ComrakPlugins};
use leptos::*;
use pulldown_cmark::CodeBlockKind;
use std::io::Write;
use std::process::Stdio;
use std::str::FromStr;
use std::{fs::OpenOptions, path::PathBuf};
use std::{io::Read, process::Command};
use tokio::fs::remove_dir_all;

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
                    "Copyright © 2023 Alejandro Osornio (AOx0). All rights reserved."
                    <br/>
                    "Made by AOx0 with "
                    <a target="_blank" rel="noopener noreferrer" href="https://github.com/gbj/leptos" class="underline">"Leptos"</a>
                    ", "
                    <a target="_blank" rel="noopener noreferrer" href="https://github.com/alpinejs/alpine" class="underline">"AlpineJS"</a>
                    " &amp; "
                    <a target="_blank" rel="noopener noreferrer" href="https://github.com/tailwindlabs/tailwindcss" class="underline">"TailwindCSS"</a>
                    ". "
                    <a href="/terms/" class="underline">"Terms"</a>
                </div>
            </div>
        </footer>
    }
}

#[component]
fn MoonIcon(cx: Scope) -> impl IntoView {
    view! {cx,
        <svg class="hidden dark:block" height="1em" viewBox="0 0 50 50" width="1em" xmlns="http://www.w3.org/2000/svg">
            <svg::path d="M 43.81 29.354 C 43.688 28.958 43.413 28.626 43.046 28.432 C 42.679 28.238 42.251 28.198 41.854 28.321 C 36.161 29.886 30.067 28.272 25.894 24.096 C 21.722 19.92 20.113 13.824 21.683 8.133 C 21.848 7.582 21.697 6.985 21.29 6.578 C 20.884 6.172 20.287 6.022 19.736 6.187 C 10.9 8.728 4.691 17.389 5.55 26.776 C 6.408 36.163 13.847 43.598 23.235 44.451 C 32.622 45.304 41.28 39.332 43.816 30.253 C 43.902 29.96 43.9 29.647 43.81 29.354 Z" fill="currentColor"/>
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
        <MenuItem href="/">"Home"</MenuItem>
        // <MenuItem href="/portfolio">"Portfolio"</MenuItem>
        <MenuItem href="/blog/">"Blog"</MenuItem>
        // <MenuItem href="/resume">"Resume"</MenuItem>
        <MenuItem href="/contact/">"Contact"</MenuItem>
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
    title: String,
    #[prop(optional)] x_data: &'static str,
    #[prop(optional)] katex: bool,
    #[prop(optional)] alpine: bool,
    #[prop(optional)] blog: bool,
    children: Box<dyn Fn(Scope) -> Fragment>,
) -> impl IntoView {
    view! { cx,
        <html class="dark">
            <head>
                <meta id="t_color" name="theme-color" content="rgb(31 41 55 / var(--tw-bg-opacity))"/>
                <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
                <title>{title}</title>
                <meta charset="UTF-8"/>
                <link rel="stylesheet" href="/static/styles.css"/>
                {(alpine || x_data != "").then(|| view!{cx, <script src=r"https://unpkg.com/alpinejs@3.x.x/dist/cdn.min.js" defer init />})}
                {blog.then(|| view!{cx, <link rel="stylesheet" href="/static/blog_styles.css"/>})}
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
                <div class="flex-auto">{children(cx)}</div>
            </body>
            <Footer/>
        </html>
    }
}

#[component]
pub fn Link(
    cx: Scope,
    #[prop(optional)] more: &'static str,
    href: &'static str,
    children: Box<dyn Fn(Scope) -> Fragment>,
) -> impl IntoView {
    view! { cx,
        <a class=format!("font-bold hover:text-orange-500 {more}") href=href>
            {children(cx)}
        </a>
    }
}

#[component]
fn H1(
    cx: Scope,
    #[prop(optional)] more: &'static str,
    children: Box<dyn Fn(Scope) -> Fragment>,
) -> impl IntoView {
    view! { cx,
        <h1 class=format!("text-4xl md:text-5xl font-bold py-10 {more}")>{children(cx)}</h1>
    }
}

#[component]
fn H2(
    cx: Scope,
    #[prop(optional)] more: &'static str,
    children: Box<dyn Fn(Scope) -> Fragment>,
) -> impl IntoView {
    view! { cx,
        <h2 class=format!("text-3xl md:text-4xl font-semibold py-5 {more}")>{children(cx)}</h2>
    }
}

#[component]
fn H3(
    cx: Scope,
    #[prop(optional)] more: &'static str,
    children: Box<dyn Fn(Scope) -> Fragment>,
) -> impl IntoView {
    view! { cx,
        <h3 class=format!("text-1xl md:text-2xl font-semibold py-4 {more}")>{children(cx)}</h3>
    }
}

#[component]
fn Caption(cx: Scope, msg: &'static str) -> impl IntoView {
    view! { cx,
        <p class="text-sm font-light block text-center pt-1">
            {msg} <br/>
        </p>
    }
}

#[component]
fn Image(cx: Scope, src: &'static str, caption: &'static str) -> impl IntoView {
    view! { cx,
        <div style="border-radius: 3pt;" class="bg-white">
            <img class="p-3" src=src/>
        </div>
        <Caption msg=caption/>

    }
}

#[component]
fn P(cx: Scope, children: Box<dyn Fn(Scope) -> Fragment>) -> impl IntoView {
    view! { cx,
        <p class="text-justify">{children(cx)}</p>
    }
}

#[component]
pub fn Tag(cx: Scope, name: &'static str, tag: &'static str) -> impl IntoView {
    view! { cx,
        <div class="relative pr-0.5">
            <button id=tag
                type="button"
                x-effect=format!("
                    if (hasValue(search, '{name}')) {{ 
                        $el.classList.remove('bg-gray-400/10'); 
                        $el.classList.remove('dark:bg-gray-900/30'); 
                        $el.classList.add('bg-gray-800/10'); 
                        $el.classList.add('dark:bg-gray-600/30'); 
                    }} else {{
                        $el.classList.add('bg-gray-400/10'); 
                        $el.classList.add('dark:bg-gray-900/30'); 
                        $el.classList.remove('bg-gray-800/10'); 
                        $el.classList.remove('dark:bg-gray-600/30'); 
                    }}
                ")
                x-on:click=format!("if (!hasValue(search, '{name}')) {{ search=addWord(search, '{name}') }} else {{ search=removeWord(search, '{name}') }}")
                class="text-gray-500 text-xs leading-5 font-semibold bg-gray-400/10 rounded-full py-1 px-3 flex
                    items-center dark:bg-gray-900/30 dark:text-gray-400 dark:shadow-highlight/4"
            >
                {name}
            </button>
        </div>
    }
}

#[component]
pub fn BlogEntryNutshell(
    cx: Scope,
    href: &'static str,
    title: &'static str,
    date: &'static str,
    des: &'static str,
    tags: &'static [(&'static str, &'static str)],
) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col" x-show="show_item($el)">
            <div class="flex">
                <Tag name=date tag=date />
                { tags.into_iter().map(|(name, tag)| view! {cx, <Tag name=name tag=tag/>}).collect::<Vec<_>>() }
            </div>
            <a href=href>
                <div class="flex justify-between items-center flex-row-revert">
                    <h2 class="blog-title font-bold text-lg pb-4 hover:text-orange-500 text-left">{title}</h2>
                </div>
            </a>
            <a href=href>
                <p class="text-justify">{des}</p>
            </a>
        </div>
    }
}

#[component]
fn Blog(cx: Scope) -> impl IntoView {
    view! { cx,
        <BaseHtml title="Blog - AOx0".to_owned() alpine=true>
            <script>
                r#"
                    function hasValue(searchIn, searchFor) {
                      const searchForWords = searchFor.split(/[ ,]+/);
                      for (const word of searchForWords) {
                        if (searchIn.includes(word)) {
                          return true;
                        }
                      }
                      return false;
                    }
                    
                    function removeWord(string, word) {
                      const parts = string.split(/(, | )/);
                      const filteredParts = parts.filter(part => part !== word && part !== `, ${word}` && part !== ` ${word}`);
                      const newString = filteredParts.join("");

                      return newString.replace(/[, ]+/g, " ").trim().replace(/^[, ]+|[, ]+$/g, "");
                    }
        
                    function addWord(string, word) {
                        const words = string.split(/[, ]+/);
                        if (!words.includes(word)) {
                            words.push(word);
                        }
                        return words.join(" ").replace(/^[, ]+|[, ]+$/g, "");
                    }
                "#
            </script>
            <div
                class="wrapper relative max-w-screen-md container text-left v-screen mx-auto pt-6 md:py-6 px-10 text-black dark:text-gray-100"
                 x-data=r#"{
                    search: '',
                    show_item(el){
                        return this.search === '' || hasValue(el.textContent.toLowerCase(), this.search.toLowerCase());
                    }
                }"#
            >
                <div class="lg:text-sm lg:leading-6 relative">
                    <div class="sticky pointer-events-none">
                        <div class="relative pointer-events-auto">
                            <div
                                class=
                                    "p-0 w-full flex items-center text-sm leading-6 text-gray-400 rounded-md ring-1 ring-gray-900/10
                                    shadow-sm py-1.5 pl-2 pr-3 bg-white dark:bg-gray-900/30 md:dark:highlight-white/5 md:dark:hover:bg-gray-800"
                            >
                                <svg width="24" height="24" fill="none" aria-hidden="true" class="mr-3 flex-none"><path d="m19 19-3.5-3.5" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path><circle cx="11" cy="11" r="6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></circle></svg>
                                <input style="-webkit-appearance: none; -webkit-border-radius:0px;" x-model="search" type="search" class="search-input h-full grow !border-none !focus:ring-0 !outline-none relative !bg-transparent rounded-none" placeholder="Quick search..."/>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="flex flex-col space-y-10 md:space-y-0">
                    <H1>"Blog"</H1>
                </div>
                <div class="flex flex-col space-y-10 md:space-y-0">
                    <BlogEntryNutshell href="./parser-comb-notes/" title="[WIP] Parser combinator notes"
                        tags=&[("Rust", "rust"), ("Parser", "parser"), ("WIP", "wip")]
                        date="2023-03-16"
                        des="These are my notes from the lecture by Scott Wlaschin, available
                            at https://www.youtube.com/watch?v=RDalzi7mhdY.
                            Parsing is a crucial task in computer science, as it allows
                            us to process and interpret data that goes into our programs.
                            There are many approaches to parsing, and parser combinators are 
                            one powerful and flexible tool for building parsers."
                    />
                    <BlogEntryNutshell href="./type-guidance/" title="Type guidance on APIs using PhantomData"
                        tags=&[("Rust", "rust")]
                        date="2022-08-06"
                        des="When writing APIs it's easy for users to make
                        misuses of methods defined within a struct. There are 
                        cases when you might want to restrict the methods available 
                        downstream depending on the state of an instance. In this writeup I 
                        talk about Rust's PhantomData and how to use it to design unbreakable APIs."
                    />
                    <BlogEntryNutshell href="./covid/" title="Data analysis exercise: COVID19 in México"
                        tags=&[("Mathematica", "mathematica")]
                        date="2021-12-25"
                        des="COVID-19 reached every place on the earth.
                        An examination of open data from México will reveal the situation there. 
                        This paper aims to describe it by showing plenty of plots and graphs, 
                        explaining how to develop them in the process.<br/><br/>The purpose, 
                        to strengthen my general analysis skills, practicing methods used to produce 
                        high-quality media."
                    />
                </div>
            </div>
        </BaseHtml>
    }
}

#[component]
fn Welcome(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="max-w-screen-md relative container text-center md:text-left v-screen mx-auto pt-6 md:py-6 px-10 text-black dark:text-gray-100">
            <h1 class="text-4xl md:text-5xl font-bold py-10 ">
                "About Me"
            </h1>
            <p class="text-justify">
                "
                    Hi 👋,<br><br>I'm Alejandro Osornio, an enthusiastic programmer who really enjoys compiled
                    languages, playing around with interpreted ones, and creating side projects of all kinds for
                    fun.<br><br>I am interested in Cyber-security, computer science, math, and Backend, enjoy writing
                    Frontend, and like writing CLI tools to make my day-to-day easier.<br><br>Currently, I'm studying
                    Data Intelligence and Cyber-security at Panamerican University.<br><br>This web page is my blog,
                    portfolio, and how to contact. Feel free to explore around and to contact me.
                "
            </p>

        </div>
    }
}

#[component]
fn Terms(cx: Scope) -> impl IntoView {
    view! {cx,
        <BaseHtml title="Terms - AOx0".to_owned()>
            <div class="max-w-screen-md relative container text-justify v-screen mx-auto pt-6 md:py-6 px-10 text-black dark:text-gray-100">
                <H1>"Terms of Service"</H1>
                <p class="text-justify">
                    r#"
                        This website is a personal portfolio owned and operated by Alejandro Osornio (the "Owner").
                        By accessing or using this website, you agree to be bound by these terms of service (these "Terms"). 
                        If you do not agree to these Terms, you may not access or use this website.
                    "#

                    <H3>"1. License to Use Website"</H3>
                    r#"
                        Subject to your compliance with these Terms, the Owner grants you a limited, non-exclusive, non-transferable, 
                        revocable license to access and use this website for your personal, non-commercial use only. 
                        This license does not include the right to: (a) sell, resell, or exploit for any commercial purposes, 
                        any portion of this website or access to this website; (b) use any data mining, robots, or similar data 
                        gathering or extraction methods; (c) download (other than the page caching) any portion of this website, 
                        except as expressly permitted on this website; or (d) use this website other than for its intended purpose.
                    "#

                    <H3>"2. Open-Source Software"</H3>
                    "This website includes the following open-source softwares that are all distributed under the MIT license: Leptos ("
                        <a href="https://github.com/gbj/leptos" class="text-blue-500 hover:underline">"https://github.com/gbj/leptos"</a>
                    "), AlpineJS ("
                        <a href="https://github.com/alpinejs/alpine" class="text-blue-500 hover:underline">"https://github.com/alpinejs/alpine"</a>
                    "), TailwindCSS ("
                        <a href="https://github.com/tailwindlabs/tailwindcss" class="text-blue-500 hover:underline">"https://github.com/tailwindlabs/tailwindcss"</a>
                    ") and Chroma ("
                        <a href="https://github.com/alecthomas/chroma" class="text-blue-500 hover:underline">"https://github.com/alecthomas/chroma"</a>
                    ")"
                    <br/>
                    r#"
                        By using this website, you agree to be bound by the terms of the applicable open-source software licenses.
                        You must include a copy of the applicable open-source software licenses and retain the copyright notice in any 
                        copies of the open-source software that you distribute. You must also provide appropriate attribution to the original 
                        authors of the open-source software as required by the terms of the applicable open-source software licenses.
                    "#

                    <H3>"3. Cookie Policy"</H3>
                    r#"
                        This website uses cookies to improve the user experience, specifically to track theme preferences. 
                        These cookies do not collect any personal information nor track your browsing activity. They are used solely for the purpose of providing a better user experience on this website. By using this website, you consent to the use of cookies.
                    "#

                    <H3>"4. Intellectual Property"</H3>
                    r#"
                        This website and all content, services, and products available on or through this website, including, but not limited 
                        to, text, graphics, logos, images, and software, are the property of the Owner or its licensors.
                        You may not use any content, services, or products on this website for any commercial 
                        purpose without the express written consent of the Owner. For non-commercial use, you must give appropriate credit 
                        to the Owner and include a link to this website.
                    "#

                    <H3>"5. User Conduct"</H3>
                    r#"
                        You agree to use this website only for lawful purposes and in a way that does not infringe the rights of, restrict, 
                        or inhibit anyone else's use and enjoyment of this website. You may not use this website in any manner that could 
                        damage, disable, overburden, or impair this website or interfere with any other party's use and enjoyment of this website.
                    "#

                    <H3>"6. Disclaimer of Warranties"</H3>
                    r#"
                        This website is provided on an "as is" and "as available" basis. The Owner makes no representations or warranties of 
                        any kind, express or implied, as to the operation of this website or the information, content, materials, or products 
                        included on this website. To the full extent permissible by law, the Owner disclaims all warranties, express or implied, 
                        including, but not limited to, implied warranties of merchantability and fitness for a particular purpose.
                    "#

                    <H3>"7. Limitation of Liability"</H3>
                    r#"
                        The Owner will not be liable for any damages of any kind arising from the use of this website, including, but not limited to, direct, indirect, incidental, punitive, and consequential damages.
                    "#

                    <H3>"Contact Information"</H3>
                    r#"
                        If you have any questions about these Terms or this website, you may contact the Owner at 
                    "#
                    <Link href="mailto:aoxo.contact@gmail.com">
                        "aoxo.contact@gmail.com"
                    </Link>
                    <p class="pt-5 text-sm">{format!("Last Updated: {}", chrono::offset::Local::now().format("%d-%m-%Y"))}</p>
                </p>

            </div>
        </BaseHtml>
    }
}

#[component]
fn Contact(cx: Scope) -> impl IntoView {
    view! { cx,
        <BaseHtml title="Contact - AOx0".to_owned()>
            <div class="max-w-screen-md relative container text-left justify-left md:text-left
                v-screen mx-auto pt-6 md:py-6 px-10 text-black dark:text-gray-100">
                <H1>"Where to find me"</H1>
                <p>"Feel free to reach me out in any of the following places:"</p>
                <ul class="list-disc list-inside pt-10">
                    <ContactItem title="Email" href="mailto:aoxo.contact@gmail.com">
                        "aoxo.contact@gmail.com"
                    </ContactItem>
                    <ContactItem title="Github" href="https://github.com/AOx0">
                        "@AOx0"
                    </ContactItem>
                    <ContactItem title="Twitter" href="https://twitter.com/AlecsOsornio">
                        "@AlecsOsornio"
                    </ContactItem>
                    <ContactItem title="LinkedIn" href="https://www.linkedin.com/in/aox0">
                        "Alejandro Osornio"
                    </ContactItem>
                    <ContactItem title="Telegram" href="https://t.me/alecz">
                        "@Alecz"
                    </ContactItem>
                    <ContactItem title="Instagram" href="https://www.instagram.com/ale.osornio/">
                        "ale.osornio"
                    </ContactItem>
                </ul>
                <p class="text-sm pt-5">
                    "* I'm most active on Telegram, though."
                </p>
            </div>
        </BaseHtml>
    }
}

#[component]
fn ContactItem(
    cx: Scope,
    title: &'static str,
    href: &'static str,
    children: Box<dyn Fn(Scope) -> Fragment>,
) -> impl IntoView {
    view! {cx,
        <li>
            <>{format!("{title}: ")}</>
            <Link href=href>
                {children(cx)}
            </Link>
        </li>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <BaseHtml title="AOx0".to_owned()>
             <Welcome/>
        </BaseHtml>
    }
}

pub fn render<F, N>(f: F) -> String
where
    F: FnOnce(Scope) -> N + 'static,
    N: IntoView,
{
    let patterns = &[
        "<!--/-->",
        "<!--#-->",
        ":shift",
        ":enter",
        ":space",
        ":ctrl",
        ":cmd",
        ":meta",
        ":alt",
        ":up",
        ":down",
        ":left",
        ":right",
        ":escape",
        ":tab",
        ":caps-lock",
        ":equal",
        ":period",
        ":slash",
        ":prevent",
        ":stop",
        ":outside",
        ":window",
        ":document",
        ":once",
        ":debounce",
        ":throttle",
        ":self",
        ":camel",
        ":dot",
        ":passive",
        ":lazy",
        ":number",
    ];
    let replacements = &[
        "",
        "",
        ".shift",
        ".enter",
        ".space",
        ".ctrl",
        ".cmd",
        ".meta",
        ".alt",
        ".up",
        ".down",
        ".left",
        ".right",
        ".escape",
        ".tab",
        ".caps-lock",
        ".equal",
        ".period",
        ".slash",
        ".prevent",
        ".stop",
        ".outside",
        ".window",
        ".document",
        ".once",
        ".debounce",
        ".throttle",
        ".self",
        ".camel",
        ".dot",
        ".passive",
        ".lazy",
        ".number",
    ];

    let rdr = "<!DOCTYPE html>".to_owned() + &render_to_string(f);
    let mut wtr = vec![];

    let ac = aho_corasick::AhoCorasick::new(patterns);
    ac.stream_replace_all(rdr.as_bytes(), &mut wtr, replacements)
        .expect("stream_replace_all failed");

    String::from_utf8(wtr).unwrap()
}

#[component]
fn MarkdownRenderer3(cx: Scope, file: PathBuf, title: String) -> impl IntoView {
    use pulldown_cmark::html;
    use pulldown_cmark::Event;
    use pulldown_cmark::Options;
    use pulldown_cmark::Parser;
    use pulldown_cmark::Tag;

    use std::io::Read;

    let mut file = OpenOptions::new().read(true).open(file).unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();

    let opts = Options::empty();
    let mut s = String::new();
    let p = Parser::new_ext(&input, opts);

    // We'll build a new vector of events since we can only consume the parser once
    let mut new_p = Vec::new();
    // As we go along, we'll want to highlight code in bundles, not lines
    let mut to_highlight = String::new();
    // And track a little bit of state
    let mut in_code_block = false;
    let mut lang = String::from_str("autodetect").unwrap();

    for event in p {
        match event {
            Event::Start(Tag::CodeBlock(a)) => {
                // In actual use you'd probably want to keep track of what language this code is
                in_code_block = true;

                if let CodeBlockKind::Fenced(a) = a {
                    lang = String::from_utf8(a.as_bytes().to_owned()).unwrap();
                    lang = lang.replace("mathematica", "Mathematica");
                }
            }
            Event::End(Tag::CodeBlock(_)) => {
                if in_code_block {
                    // Format the whole multi-line code block as HTML all at once
                    let mut child = Command::new("chroma")
                        .args([
                            &format!(r#"--lexer={}"#, lang),
                            r#"--style=github-dark"#,
                            r#"--html"#,
                            r#"--html-only"#,
                        ])
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .spawn()
                        .unwrap();

                    let child_stdin = child.stdin.as_mut().unwrap();
                    child_stdin.write_all(&to_highlight.as_bytes()).unwrap();
                    drop(child_stdin);

                    let output = child.wait_with_output().unwrap();

                    let mut child = Command::new("ruplacer")
                        .args([r#"class="([a-zA-Z0-9]+)""#, r#"class="dark:$1 $1""#, r"-"])
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .spawn()
                        .unwrap();

                    let child_stdin = child.stdin.as_mut().unwrap();
                    child_stdin.write_all(&output.stdout).unwrap();
                    drop(child_stdin);

                    let output = child.wait_with_output().unwrap();

                    let html = String::from_utf8(output.stdout).unwrap();
                    // And put it into the vector
                    new_p.push(Event::Html(html.into()));
                    to_highlight = String::new();
                    in_code_block = false;
                }
            }
            Event::Text(t) => {
                if in_code_block {
                    // If we're in a code block, build up the string of text
                    to_highlight.push_str(&t);
                } else {
                    new_p.push(Event::Text(t))
                }
            }
            e => {
                new_p.push(e);
            }
        }
    }

    // Now we send this new vector of events off to be transformed into HTML
    html::push_html(&mut s, new_p.into_iter());

    view! { cx,
        <BaseHtml title=format!("{}{} - AOx0", &title.to_uppercase().chars().into_iter().next().unwrap(), &title[1..].to_owned().replace("-", " ") ) katex=true blog=true>
            <div class="max-w-screen-md relative container text-justify md:text-left v-screen mx-auto pt-6 md:py-6 px-10 text-black dark:text-gray-100">
                {s.to_owned()}
            </div>
        </BaseHtml>
    }
}

#[component]
fn MarkdownRenderer2(cx: Scope, file: PathBuf, title: String) -> impl IntoView {
    use pulldown_cmark::{html, Parser};
    let mut file = OpenOptions::new().read(true).open(file).unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();

    let parser = Parser::new(&input);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    view! { cx,
        <BaseHtml title=format!("{}{} - AOx0", &title.to_uppercase().chars().into_iter().next().unwrap(), &title[1..] ) katex=true blog=true>
            <div class="max-w-screen-md relative container text-justify md:text-left v-screen mx-auto pt-6 md:py-6 px-10 text-black dark:text-gray-100">
                {html_output.to_owned()}
            </div>
        </BaseHtml>
    }
}

#[component]
fn MarkdownRenderer(cx: Scope, file: PathBuf, title: String) -> impl IntoView {
    let mut file = OpenOptions::new().read(true).open(file).unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();

    let adapter = SyntectAdapter::new("InspiredGitHub");
    let options = ComrakOptions::default();
    let mut plugins = ComrakPlugins::default();

    plugins.render.codefence_syntax_highlighter = Some(&adapter);
    let formatted = markdown_to_html_with_plugins(&input, &options, &plugins);

    view! { cx,
        <BaseHtml title=format!("{} - AOx0", title.to_uppercase().replace("-", " ")) katex=true >
            <div class="max-w-screen-md relative container text-center md:text-left v-screen mx-auto pt-6 md:py-6 px-10 text-black dark:text-gray-100">
                {formatted.to_owned()}
            </div>
        </BaseHtml>
    }
}

async fn show_contact() -> Html<String> {
    Html(render(|cx| view! {cx, <Contact /> }))
}

async fn show_terms() -> Html<String> {
    Html(render(|cx| view! {cx, <Terms /> }))
}

async fn show_blog() -> Html<String> {
    Html(render(|cx| view! {cx, <Blog /> }))
}

async fn show_blog_entry(Path(name): Path<String>) -> Html<String> {
    let file = format!("{}/blog/{}.md", env!("CARGO_MANIFEST_DIR"), name);
    Html(render(move |cx| {
        view! {cx,
            <MarkdownRenderer3
                title=name
                file=PathBuf::from_str(&file).unwrap()
            />
        }
    }))
}

async fn say_hello() -> Html<String> {
    Html(render(|cx| view! {cx, <Home /> }))
}

fn set_return_type<T, F: std::future::Future<Output = T>>(_arg: &F) {}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

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
        .route("/contact/", get(show_contact))
        .route("/terms/", get(show_terms))
        .route("/blog/:name/", get(show_blog_entry))
        .route("/blog/", get(show_blog))
        .nest_service("/static/", static_service);

    let port = "8000";

    if args.len() == 1 {
        let (txs, rxs) = tokio::sync::oneshot::channel::<()>();

        let renderer = async move {
            let current = format!("{}/target", std::env!("CARGO_MANIFEST_DIR"));
            let out_dir = format!("{current}/0.0.0.0");

            if PathBuf::from(&out_dir).exists() {
                println!("Removing old {out_dir}");
                remove_dir_all(&out_dir).await?;
            }
            println!("Executing commands");

            println!("    Suckit");
            Command::new("suckit")
                .args(format!("http://0.0.0.0:{port}/ -j 8 -o {current}",).split_whitespace())
                .status()?;
            println!("    Replace index.html -> ./");
            Command::new("ruplacer")
                .args(format!("index.html ./ {out_dir} --quiet --go").split_whitespace())
                .status()?;

            println!("    Replace ../url.algo -> https://url.algo");
            Command::new("ruplacer")
                .args(
                    format!(
                        r#"(\.\./)+([a-zA-Z\-\.]+)(\.com|\.me|\.net|\.org|\.mx)/ https://$2$3/ {out_dir} --quiet --go"#
                    )
                    .split_whitespace(),
                )
                .status()?;

            println!("    Replace index_no_slash.html -> ");
            Command::new("ruplacer")
                .args(["index_no_slash.html", "", &out_dir, "--quiet", "--go"])
                .status()?;

            println!("    Replace \"/./\" -> \"/\"");
            Command::new("ruplacer")
                .args([r#"/\./""#, r#"/""#, &out_dir, "--quiet", "--go"])
                .status()?;

            println!("    Replace syntax-something -> dark:syntax-something syntax-something");
            Command::new("ruplacer")
                .args([
                    r#"syntax-([a-zA-Z]+)"#,
                    "$0 dark:$0",
                    &out_dir,
                    "-t",
                    "*.html",
                    "--quiet",
                    "--go",
                ])
                .status()?;
            txs.send(()).unwrap();
            Ok(())
        };
        set_return_type::<Result<()>, _>(&renderer);

        let a = tokio::spawn(renderer);

        let server =
            Server::bind(&format!("0.0.0.0:{port}").parse()?).serve(app.into_make_service());

        let graceful = server.with_graceful_shutdown(async move {
            println!("Starting Axum Server");
            rxs.await.ok();
            println!("Ending Axum Server");
        });

        graceful.await?;
        println!("Axum Server Ended");
        a.await??;
        Ok(())
    } else {
        Server::bind(&format!("0.0.0.0:{port}").parse()?)
            .serve(app.into_make_service())
            .await?;
        Ok(())
    }
}
