use yew::prelude::*;

#[function_component(Banner)]
pub fn banner() -> Html {
    html! {
    <section class="bg-gray-50">
      <div
        class="mx-auto max-w-screen-xl px-4 py-32 lg:flex lg:h-screen lg:items-center">
        <div class="mx-auto max-w-xl text-center">
          <h1 class="text-3xl text-slate-800 font-extrabold sm:text-5xl">
              {"Saeed Karimi"}
            <strong class="font-extrabold text-sky-600 text-lg sm:block">
                {"Fullstack Software Developer"}
            </strong>
          </h1>

          <p class="mt-4 sm:text-xl/relaxed">
              {"I'm a fullstack software developer based in Vancouver, Canada. I love to build web applications and I'm passionate about learning new technologies."}
          </p>

          <div class="mt-8 flex flex-wrap justify-center gap-4">
            <a
              class="block w-full rounded bg-sky-600 px-12 py-3 text-sm font-medium text-white shadow hover:bg-sky-700 focus:outline-none focus:ring active:bg-sky-500 sm:w-auto"
              href="/get-started"
            >
                {"Get In Touch"}
            </a>

            <a
              class="block w-full rounded px-12 py-3 text-sm font-medium text-sky-600 shadow hover:text-sky-700 focus:outline-none focus:ring active:text-sky-500 sm:w-auto"
              href="/about"
            >
                {"About Me"}
            </a>
          </div>
        </div>
      </div>
    </section>

    }
}
