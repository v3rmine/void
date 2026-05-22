function process_wild_mode() {
    let is_wild_mode = localStorage.getItem("wild-mode") === "true";

    // Hide all wild/not-wild elements
    document.querySelectorAll('.wild').forEach(function (entry) {
        if (is_wild_mode) {
            entry.removeAttribute('hidden');
            const parser = new DOMParser();
            const doc = parser.parseFromString(
                new TextDecoder()
                    .decode(Uint8Array.fromBase64(entry.getAttribute("data-wild")),
                    ), "text/html");
            entry.innerHTML = '';
            while (doc.body.firstChild) {
                entry.appendChild(doc.body.firstChild);
            }

            add_onclick_listeners(entry);
        } else {
            entry.setAttribute('hidden', true);
            entry.innerHTML = "";
        }
    });

    document.querySelectorAll('.not-wild').forEach(function (entry) {
        if (is_wild_mode) {
            entry.setAttribute('hidden', true);
        } else {
            entry.removeAttribute('hidden');
        }
    });

    document.querySelectorAll('.wild-link').forEach(function (entry) {
        if (is_wild_mode) {
            entry.removeAttribute('disabled');
        } else {
            entry.setAttribute('disabled', true);
        }
    });
}

function add_onclick_listeners(element = document) {
    element.querySelectorAll('.toggle-wild').forEach(function (entry) {
        entry.addEventListener("click", function (event) {
            event.stopImmediatePropagation();

            if (localStorage.getItem("wild-mode") === "true") {
                localStorage.removeItem("wild-mode");
            } else {
                localStorage.setItem("wild-mode", "true");
            }

            process_wild_mode();
        });
    });
}

let url = new URL(location.href);
if (Boolean(url.searchParams.get("wild"))) {
    localStorage.setItem("wild-mode", "true")
}

process_wild_mode();
add_onclick_listeners();
