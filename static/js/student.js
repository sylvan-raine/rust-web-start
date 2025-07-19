"use strict";

import { auth_fetch } from "./auth.js";
import { param_string } from "./util.js";

const query_btn = document.querySelector("#query");
query_btn.addEventListener("click", handle_query);

const keyword_ = document.querySelector("#keyword");
const department_ = document.querySelector("#department");
const email_ = document.querySelector("#email");
const age_ = document.querySelector("#age");
const page_size_ = document.querySelector("#page-size");
const page_index_ = document.querySelector("#page-index");
const data_table = document.querySelector("#data");

async function handle_query() {
    const params = collect_input();
    const response = await auth_fetch("GET", "/api/student/query" + param_string(params));
    
    const page = await response.json();
    const table_body = data_table.getElementsByTagName("tbody")[0];

    page.items.forEach((student) => {
        const tr = student_to_table_row(student);
        table_body.appendChild(tr)
    });
}

function collect_input() {
    const keyword = keyword_.value;
    const department = department_.value;
    const email = email_.value;
    const age = age_.value;
    const size = page_size_.value;
    const index = page_index_.value;

    return {
        keyword,
        department,
        email,
        age,
        size,
        index
    };
}

function student_to_table_row(student) {
    const tr = document.createElement("tr");
    Object.keys(student)
        .forEach((key) => {
            const new_cell = tr.insertCell();
            if (key == "id") {
                new_cell.scope = "row";
            }
            new_cell.textContent = student[key];
        });
    return tr;
}
