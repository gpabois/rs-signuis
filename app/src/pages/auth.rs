use leptos::{component, view, IntoView};

#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <main class="w-full flex items-center justify-center pt-8">
            <div>
            <form method="post" action="/login" class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
                <div class="mb-4">
                <label class="block text-gray-700 text-sm font-bold mb-2" for="username_or_email">"Nom d'utilisateur / Adresse courriel"</label>
                <input
                    class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                    type="text"
                    id="username_or_email"
                />
                </div>
                <div class="mb-4">
                <label class="block text-gray-700 text-sm font-bold mb-2" for="password">"Mot de passe"</label>
                <input
                    class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                    type="password"
                    id="password"
                />
                </div>
                <div class="flex items-center justify-between">
                    <input
                        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                        type="button" value="Se connecter"/>
                    </div>
            </form>
            <p class="text-center text-gray-500 text-xs">
                "© 2024 Gaël Pabois"
            </p>
            </div>
        </main>
    }
}
