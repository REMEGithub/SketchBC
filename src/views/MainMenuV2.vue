<template>
    <header>
        <div class="background-panel-header">
            <Header />
        </div>
    </header>
	<main>
        <div class="fileChooser">
            <FileButton />
        </div>
        <div class="modeChooser">
            <section class="toggle-btn">
                <SideToggleGroup />
            </section>
            <section class="timetoggle-btn">
                <TimeToggleBtnGroup />
                <PageJumpButton label="开始绘制" :to="pname" />
            </section>
        </div>
    </main>
    <footer>
        <div class="background-panel-footer"></div>
    </footer>
</template>

<script setup>
import SideToggleGroup from '@/components/common/SideToggleGroup.vue';
import FileButton from '@/components/common/FileButton.vue';
import TimeToggleBtnGroup from '@/components/common/TimeToggleBtnGroup.vue';
import PageJumpButton from '@/components/common/PageJumpButton.vue';
import Header from '@/components/common/Header.vue';

const pname = {name:'exhibit'}
</script>

<style scoped>
header {
    display: flex;
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 125px;
    background: rgba(228, 228, 228, 1);
}
main {
    display: flex;
    position:absolute;
    flex-direction: column;
    top: 125px;
    left: 0;
    width: 100%;
    height: calc(100vh - (125px + 102px));
    background: rgba(228, 228, 228, 1);
}
footer {
    display: flex;
    position:absolute;
    bottom: 0;
    left: 0;
    width: 100%;
    height: 102px;
}
.fileChooser {
    display: flex;
    align-items: center;
    justify-content: center;
    margin: 0 0 0 44px;
    width: 709px;
    height: 174px;
}
.modeChooser {
    display: flex;
    /* justify-content: center; */
    margin: 47px 0 0 44px;
    width: 703px;
    height: 315px;
    border-radius: 16px;
    
}
.toggle-btn {
    width: 212px;
    height: 315px;
    border-radius: 12px;
}

.timetoggle-btn {
    width: 454px;
    height: 315px;
    border-radius: 12px;
    margin-left: 43px;
}


.background-panel-footer {
    background-image: url("@/assets/imgSrc/home_bg.png");
    background-size: 100% 100%;
    background-repeat: no-repeat;
    background-position: center;

    width: 100%;
    height: 102px;
}

.background-panel-header {
    background-image: url("@/assets/imgSrc/home_toast.png");
    background-size: 100% 100%;
    background-repeat: no-repeat;
    background-position: center;

    display: flex;
    position: relative;

    width: 100%;
    height: 115.2px;
}


</style>
