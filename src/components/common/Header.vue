<template>
    <div class="header-date">
        <span>{{ formattedDate }}</span>
    </div>
    <div class= "header-logo">
        <img src="@/assets/imgSrc/home_cat2.png" alt="logo" />
    </div>
    <div class="header-coin">
        <img src="@/assets/imgSrc/home_coin.png" alt="coin" />
        <span>1258</span>
        <span class="coinplus">+1</span>
    </div>
</template>

<script setup lang="ts">
import dayjs from 'dayjs';

const now = dayjs();
const formattedDate = now.format('MM月DD日');
</script>

<style scoped>
.header-date {
    display: flex;
    position: absolute;
    top:25px;
    left: 36px;

    min-width: 89px;
    min-height: 34px;
    font-size: 28px;
    font-weight: 500;
}
.header-logo {
    display: flex;
    justify-content: center;

    position: absolute;
    top:0px;
    left: 358px;

    min-width: 83px;
    min-height: 83px;
}
.header-coin {
    display: flex;
    align-items: center;
    position: absolute;
    top: 20px;
    right: 38px;

    min-width: 89px;
    min-height: 34px;
}
.header-coin span {
    font-weight: 500;
    font-size:28px;
}
.coinplus {
    color: rgba(195, 195, 195, 1);
}
</style>