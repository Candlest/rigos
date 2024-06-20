// script.js
function openTab(evt, tabName) {
    var i, tabcontent, tablinks;

    // 获取所有tabcontent元素
    tabcontent = document.getElementsByClassName("tabcontent");
    for (i = 0; i < tabcontent.length; i++) {
        tabcontent[i].style.display = "none";
    }

    // 获取所有tablinks元素
    tablinks = document.getElementsByClassName("tablinks");
    for (i = 0; i < tablinks.length; i++) {
        tablinks[i].className = tablinks[i].className.replace(" active", "");
    }

    // 显示当前的tabcontent并添加active类
    document.getElementById(tabName).style.display = "block";
    evt.currentTarget.className += " active";
}