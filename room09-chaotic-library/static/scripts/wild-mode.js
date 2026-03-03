window.addEventListener('load', function () {
  let is_wild_mode = localStorage.getItem("wild-mode") === "true";

  // Hide all wild/not-wild elements
  document.querySelectorAll('.wild').forEach(function (entry) {
    if (is_wild_mode) {
      entry.removeAttribute('hidden');
      const parser = new DOMParser();
      const doc = parser.parseFromString(
        new TextDecoder()
          .decode(Uint8Array.fromBase64(entry.textContent),
      ), "text/html");
      entry.innerHTML = '';
      while (doc.body.firstChild) {
        entry.appendChild(doc.body.firstChild);
      }
    } else {
      entry.setAttribute('hidden', true);
      entry.innerHTML = window.atob(entry.innerHTML);
    }
  });
  document.querySelectorAll('.not-wild').forEach(function (entry) {
    if (is_wild_mode) {
      entry.setAttribute('hidden', true);
    } else {
      entry.removeAttribute('hidden');
    }
  });

  document.querySelectorAll('.toggle-wild').forEach(function (entry) {
    entry.addEventListener('click', function () {
      if (localStorage.getItem("wild-mode") === "true") {
        localStorage.removeItem("wild-mode");
      } else {
        localStorage.setItem("wild-mode", "true");
      }

      location.reload();
    });
  });
});
