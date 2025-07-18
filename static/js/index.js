"use strict";

import { login, token_legal } from "./auth.js";

const overlay = document.querySelector(".overlay");
const handin_btn = document.querySelector("#submit-information");
const login_btn = document.querySelector("#login-button");
const login_window = document.querySelector("#login-window");
const cancel_button = document.querySelector("#cancel-button");

handin_btn.addEventListener("click", handle_login);
login_btn.addEventListener("click", open_login_window);
cancel_button.addEventListener("click", close_login_window);
document.addEventListener("DOMContentLoaded", suggest_login);

async function handle_login() {
    const id = document.getElementById("user-id").value;
    const password = document.getElementById("user-passwd").value;

    const success_callback = () => {
        alert("您已成功登录!");
        close_login_window();
    };

    const failure_callback = () => alert("登陆失败! 请重试!");

    login(id, password)
        .then(success_callback, failure_callback)
        .catch(() => { alert("登陆失败! 请重试!"); })
}

function open_login_window() {
    overlay.classList.remove("hidden");
    login_window.classList.remove("hidden");
}

function close_login_window() {
    overlay.classList.add("hidden");
    login_window.classList.add("hidden");
}

async function suggest_login() {
    if (token_legal()) {
        close_login_window();
    } else {
        open_login_window();
    }
}