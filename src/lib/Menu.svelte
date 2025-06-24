<nav class="bottom-nav">
    <div class="nav-activ">
        <button class="">
            <i class="icon-home"></i>
        </button>
    </div>
    
    <a href="#" class="nav-item">
        <i class="icon-user"></i>
    </a>
    <a href="#" class="nav-item active">
        <div class="bell-container">
            <i class="icon-bell"></i>
            <span class="notification-badge">1</span>
        </div>
    </a>
    <a href="#" class="nav-item">
        <i class="icon-menu"></i>
    </a>
</nav>


<style>
    .bottom-nav {
    position: fixed;
    bottom: 0;
    left: 0;
    width: 100%;
    height: 70px;
    background-color: #2A2A2A;
    display: flex;
    justify-content: space-around;
    align-items: center;
    border-top-left-radius: 20px;
    border-top-right-radius: 20px;
    box-shadow: 0 -5px 10px rgba(0, 0, 0, 0.3);
}

.nav-item {
    text-align: center;
    flex: 1;
}

.nav-item i {
    font-size: 24px;
    color: #ccc;
}

.nav-activ {


    position: absolute;
    width: 110px;
    height: 55.5px;
    left: 14px;
    top: 11px;

    background: #111111;

}
.nav-item.active .bell-container {
    position: relative;
    top: -35px;
    width: 70px;
    height: 70px;
    background-color: #00FF00; /* Зеленый фон */
    border-radius: 50%;
    display: flex;
    justify-content: center;
    align-items: center;
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.3);
}

.bell-container {
    position: relative;
}

.notification-badge {
    position: absolute;
    top: -5px;
    right: -5px;
    width: 20px;
    height: 20px;
    background-color: red;
    color: white;
    border-radius: 50%;
    font-size: 14px;
    display: flex;
    justify-content: center;
    align-items: center;
}
</style>