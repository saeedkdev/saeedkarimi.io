use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <nav
          aria-label="Site Nav"
          class="w-full font-open-sans">
          <div class="mx-auto flex w-11/12 items-center justify-between p-4 border-b border-slate-300">
              <a href="/" class="inline-flex h-10 text-2xl items-center justify-center font-bold">
                <span class="sr-only">{"SK"}</span>
                {"SKDEV"}
              </a>

              <ul class="flex items-center gap-2 text-sm font-medium text-gray-500">
                <li>
                  <a
                    class="inline-flex items-center gap-2 rounded-lg px-3 py-2"
                    href="https://github.com/saeedkdev"
                    target="_blank">
                    {"GitHub"}

                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      fill="none"
                      viewBox="0 0 24 24"
                      stroke="currentColor"
                      class="h-4 w-4">
                      <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"
                      ></path>
                    </svg>
                  </a>
                </li>
              </ul>
          </div>
        </nav>

    }
}
