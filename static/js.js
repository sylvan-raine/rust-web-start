const ico = document.querySelector(".ico");
ico.addEventListener("mouseover", () => {
    console.log("Passed by");
    ico.classList.add("cyan");
});
ico.addEventListener("mouseout", () => {
    ico.classList.remove("cyan");
});