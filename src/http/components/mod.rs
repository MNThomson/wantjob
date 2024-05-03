use leptos::{component, view, Children, IntoView};
use minify_html::{minify, Cfg};

use super::index::JobDetails;

#[tracing::instrument(skip_all)]
fn minify_html(str: String) -> String {
    return String::from_utf8(minify(str.as_bytes(), &Cfg::new())).unwrap();
}

#[tracing::instrument(skip_all)]
pub fn htmlify<F, N>(f: F) -> String
where
    F: FnOnce() -> N + 'static,
    N: IntoView,
{
    let ssr = leptos::ssr::render_to_string(f).to_string();

    return minify_html(ssr);
}

// https://fonts.googleapis.com/css?family=Zen%20Maru%20Gothic:400,700&subset=latin
/*
const FONTSTYLE: &str = r###"
@font-face {
    font-family: 'Fira Code';
    font-style: normal;
    font-weight: 400;
    src: local("Fira Code"), url(/static/fonts/FiraCode400.woff2) format('woff2');
    font-display: swap;
}
@font-face {
    font-family: 'Fira Code';
    font-style: normal;
    font-weight: 700;
    src: local("Fira Code"), url(/static/fonts/FiraCode700.woff2) format('woff2');
    font-display: swap;
}
"###;
*/
const FONTSTYLE: &str = r###"
@font-face {
    font-family: 'Sofia Pro';
    font-style: normal;
    font-weight: 400;
    src: local("Sofia Pro"), url(/static/fonts/SofiaPro400.woff) format('woff');
    font-display: swap;
}
@font-face {
    font-family: 'Sofia Pro';
    font-style: normal;
    font-weight: 700;
    src: local("Sofia Pro"), url(/static/fonts/SofiaPro700.woff) format('woff');
    font-display: swap;
}
"###;

// https://coolors.co/252422-95340e-333947-818aa3-fffcf2
const TAILWINDCONFIG: &str = r###"
tailwind.config = {
    theme: {
        extend: {
            colors: {
                // "light-weakest": "#737373",
                // "light-weak": "#a3a3a3",
                light: "#e5e5e5",
                dark: "#151514",
                "dark-weak": "#252422",
                accent: "#A83B10",
                // "accent-weak": "#65a30d",
                // "accent-weakest": "#4d7c0f",
                "link": "#818AA3",
            },
            dropShadow: {
                "colored-sm": "0 0 1px var(--tw-shadow-color)",
                "colored": "0 0 2px var(--tw-shadow-color)",
                "colored-md": "0 0 3px var(--tw-shadow-color)",
                "colored-lg": "0 0 8px var(--tw-shadow-color)",
            }
        },
        fontFamily: {
            sans: ["Sofia Pro", "Lato", "sans-serif"],
        }
    }
};"###;

#[component]
fn NavLink(#[prop(into)] title: String, #[prop(into)] url: String) -> impl IntoView {
    return view! {
        <a href={url} class="text-gray-300 hover:text-gray-100">{title}</a>
    };
}

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    return view! {
        <html lang="en">
            <head>
                <title>Want Job</title>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <link rel="icon" href="/static/favicon.svg"/>
                <script src="https://unpkg.com/htmx.org@1.9.9/dist/htmx.min.js" />
                <script src="https://cdn.tailwindcss.com/3.3.5"></script>
                <script inner_html={TAILWINDCONFIG.to_string()}></script>
                <style inner_html={FONTSTYLE.to_string()}></style>
            </head>

            <body class="h-screen bg-dark text-light text-base font-sans m-0">
                <header class="py-3 bg-dark-weak shadow-md">
                    <nav class="container flex mx-auto justify-between items-center px-2 xl:max-w-screen-xl lg:max-w-screen-lg md:max-w-screen-md">
                        <div class="flex items-center">
                            <a href="/" class="flex text-accent text-center text-3xl font-bold">
                                <img
                                    src="/static/favicon.svg"
                                    class="max-h-10 inline mr-2 drop-shadow-colored"
                                /><span class="drop-shadow-colored">WantJob</span>
                            </a>

                            /*
                            <div class="flex space-x-4 ml-5">
                                <NavLink title="Jobs" url="#" />
                                <NavLink title="Saved" url="#" />
                                <NavLink title="Applied" url="#" />
                            </div>
                            */
                        </div>
                        // Spacing: 1fr
                        <div class="flex center-items space-x-4">
                                <NavLink title="Login" url="#" />
                                <NavLink title="Register" url="#" />
                        </div>
                    </nav>
                </header>

                <main class="mb-auto mx-auto mt-2 max-w-screen-xl md:text-lg text-base">
                    {children()}
                </main>

                /*
                <footer class="footer">
                    <div class="container mx-auto text-center">
                        <a href="https://github.com/MNThomson/wantjob" class="text-secondary" target="_blank" rel="noopener noreferrer">
                            <small class="text-muted">Powered by WantJob | Made with {"♥️"}</small>
                        </a>
                    </div>
                </footer>
                */
            </body>
        </html>
    };
}

#[component]
pub fn CompanyLogo(#[prop(into)] name: String) -> impl IntoView {
    return view! {
        <img src="https://logo.clearbit.com/{name}" class="h-16 rounded-lg mr-4" />
    };
}

#[component]
pub fn JobSummary(job: JobDetails) -> impl IntoView {
    return view! {
        <a hx-get=format!("/htmx/jobs/details/{}", job.id) hx-target="#job-details" class="bg-dark-weak flex p-2 rounded-sm hover:bg-white/10 cursor-pointer mb-1">
            <CompanyLogo name={job.company.to_owned() + ".com"} />
            <div class="grow">
                <div class="flex grow justify-between">
                    <p class="underline text-link font-bold">{job.title}</p>
                    <p>{job.company}</p>
                </div>
                <div class="flex grow justify-between">
                    <p>{job.location}</p>
                    <p class="rounded-sm px-2 text-[#FFFCF2] bg-accent">{job.salary}</p>
                    //<a href={company} class="text-gray-300 hover:text-gray-100">{title}</a>
                </div>
            </div>
        </a>
    };
}

#[component]
pub fn JobPostingDetails(job: JobDetails) -> impl IntoView {
    return view! {
        <div id="job-details">
            <p class="font-bold text-3xl">{job.title}</p>
            <p>{job.description}</p>
        </div>
    };
}

#[component]
pub fn HomePageDetails() -> impl IntoView {
    return view! {
        <p class="font-bold text-4xl text-center">Welcome to WantJob!</p>
    };
}
