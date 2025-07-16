"use strict";

import { get_token, JWT_KEY } from "./auth.js";

const overlay = document.querySelector(".overlay");
const handin_btn = document.querySelector("#handin-information");
const login_btn = document.querySelector("#login-button");
const login_window = document.querySelector("#login-window");
const cancel_button = document.querySelector("#cancel-button");


handin_btn.addEventListener("click", try_login);
login_btn.addEventListener("click", open_login_window);
cancel_button.addEventListener("click", close_login_window);
document.addEventListener("DOMContentLoaded", suggest_login);

async function try_login() {
    const success = await handin_information();
    if (success) {
        close_login_window();
    }
}

function open_login_window() {
    overlay.classList.remove("hidden");
    login_window.classList.remove("hidden");
}

function close_login_window() {
    overlay.classList.add("hidden");
    login_window.classList.add("hidden");
}

async function handin_information() {
    const id = document.getElementById("user-id").value;
    const password = document.getElementById("user-passwd").value;

    const data = {
        id, password
    };

    const json = JSON.stringify(data);

    try {
        const response = await fetch('/api/login', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: json
        });

        if (response.ok) {
            const token = await response.json();
            localStorage.setItem(JWT_KEY, token);
            return true;
        } else {
            console.log("登录失败", response);
            return false;
        }
    } catch (error) {
        console.error('错误:', error);
        return false;
    }
}

async function suggest_login() {
    if (get_token()) {
        close_login_window();
    } else {
        open_login_window();
    }
}