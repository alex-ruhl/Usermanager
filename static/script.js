$(document).ready(function () {
    $("#edit").on("click", function () {
        var id = $('#idEdit').text();
        var name = $('#nameEdit').text();
        var surname = $('#surnameEdit').text();
        var email = $('#emailEdit').text();
        var pw = $('#pwEdit').text();
        var role = $('#roleEdit').text();
        console.log("Button pressed");
        $.ajax({
            type: "POST",
            url: "/edit?"
            + "id=" + id + "&name=" + name + "&surname=" + surname + "&email=" + email+ "&pw=" + pw + "&role=" + role,
            success: function (data) {
            },
            error: function (xhr, ajaxOptions, thrownError) {
            }
        })
    });
});
