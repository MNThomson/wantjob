use leptos::*;

// https://fonts.googleapis.com/css?family=Zen%20Maru%20Gothic:400,700&subset=latin
const FONTSTYLE: &str = r###"
@font-face {
    font-family: 'Zen Maru Gothic';
    font-style: normal;
    font-weight: 400;
    src: local("Zen Maru Gothic"), url(/static/fonts/ZenMaruGothic400.woff2) format('woff2');
    font-display: swap;
}
@font-face {
    font-family: 'Zen Maru Gothic';
    font-style: normal;
    font-weight: 700;
    src: local("Zen Maru Gothic"), url(/static/fonts/ZenMaruGothic700.woff2) format('woff2');
    font-display: swap;
}
"###;

const TAILWINDCONFIG: &str = r###"
tailwind.config = {
    theme: {
        extend: {
            colors: {
                // "light-weakest": "#737373",
                // "light-weak": "#a3a3a3",
                light: "#e5e5e5",
                dark: "#171717",
                "dark-weak": "#262626",
                accent: "#84cc16",
                // "accent-weak": "#65a30d",
                // "accent-weakest": "#4d7c0f",
            },
            dropShadow: {
                "colored-sm": "0 0 1px var(--tw-shadow-color)",
                "colored": "0 0 2px var(--tw-shadow-color)",
                "colored-md": "0 0 3px var(--tw-shadow-color)",
                "colored-lg": "0 0 8px var(--tw-shadow-color)",
            }
        },
        fontFamily: {
            sans: ["Zen Maru Gothic", "Lato", "sans-serif"],
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

            <body class="flex flex-col h-screen justify-between bg-dark text-light text-base font-sans m-0">
                <header class="py-3 bg-dark-weak shadow-md">
                    <nav class="container flex mx-auto justify-between items-center px-10 xl:max-w-screen-xl lg:max-w-screen-lg md:max-w-screen-md">
                        <div class="flex items-center">
                            <a href="/" class="text-accent text-center text-xl font-bold">
                                <img
                                    src="/static/favicon.svg"
                                    class="max-h-8 inline mr-2 drop-shadow-colored"
                                /><span class="drop-shadow-colored">WantJob</span>
                            </a>

                            <div class="flex space-x-4 ml-5">
                                <NavLink title="Users" url="#" />
                                <NavLink title="Scoreboad" url="#" />
                                <NavLink title="Challenges" url="#" />
                            </div>
                        </div>
                        // Spacing: 1fr
                        <div class="flex center-items space-x-4">
                                <NavLink title="Login" url="#" />
                                <NavLink title="Register" url="#" />
                        </div>
                    </nav>
                </header>

                <main class="mb-auto mx-auto mt-2 max-w-screen-md md:text-lg text-base">
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
