const { invoke } = window.__TAURI__.tauri;

const BASE_URL = "http://localhost:8080";

function toggle_tree(id) {
  fetch(`${BASE_URL}/tree/${id}`, { method: "PATCH" })
    .then((response) => response.json())
    .then((res) => {
      console.log(`rows affected: ${res.rows_affected}`);
      const tree_item = document.getElementById(id);
      tree_item.classList.toggle("completed");
    });
}

function delete_tree(id) {
  fetch(`${BASE_URL}/tree/${id}`, { method: "DELETE" })
    .then((response) => response.json())
    .then((res) => {
      console.log(`rows affected: ${res.rows_affected}`);
      let elm = document.getElementById(id);
      elm.remove();
    });
}

function add_tree(title) {
  console.log(`adding tree: ${title}`);
  fetch(`${BASE_URL}/tree/${title}`, { method: "POST" })
    .then((response) => response.json())
    .then((tree) => {
      const tree_list = document.getElementById("tree-list");
      tree_list.appendChild(construct_tree(tree));
    });
}

function construct_tree(tree) {
  var input = document.createElement('input');
  input.type = "checkbox";
  input.checked = tree.completed;

  input.onclick = (e) => {
    toggle_tree(tree.id)
  }

  var label = document.createElement('label');
  label.innerHTML = tree.title;

  var button = document.createElement('button');
  button.innerHTML = "Delete";

  button.onclick = (e) => {
    delete_tree(tree.id)
  }

  var tree_item = document.createElement('li');
  tree_item.classList.add("center");
  if (tree.completed) {
    tree_item.classList.add("completed");
  }
  tree_item.id = tree.id;
  tree_item.appendChild(input);
  tree_item.appendChild(label);
  tree_item.appendChild(button);

  return tree_item;
}

window.addEventListener("DOMContentLoaded", () => {
  window.__APP_STATE__ = {
    trees: []
  };

  let new_tree_input = document.getElementById("new-tree-input");
  let new_tree_btn = document.getElementById("new-tree-btn");
  new_tree_btn.onclick = (e) => {
    add_tree(new_tree_input.value)
    new_tree_input.value = "";
  }

  const tree_list = document.getElementById("tree-list");

  fetch(`${BASE_URL}/trees`)
    .then((response) => response.json())
    .then((trees) => {
      trees.forEach((tree) => {
        tree_list.appendChild(construct_tree(tree));
      });
      console.log(trees);
    });
});
