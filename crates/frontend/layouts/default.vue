<template>
    <v-app theme="dark">
        <v-navigation-drawer rail v-model="drawerLeft" location="left" permanent>
            <v-list class="pa-0">
                <v-list-item v-for="(item,i) in navbar" :key="'nb'+i" :to="item.to">
                    <v-list-item-avatar start>
                        <v-icon :icon="item.icon"></v-icon>
                    </v-list-item-avatar>
                    <v-list-item-title> {{item.title}}</v-list-item-title>
                </v-list-item>
            </v-list>
        </v-navigation-drawer>
        <v-app-bar density="compact">
            <v-app-bar-nav-icon @click="drawerLeft = !drawerLeft"></v-app-bar-nav-icon>
            <v-spacer></v-spacer>
            <v-text-field hide-details single-line variant="underlined" placeholder="Search" class="hdy-searchbar" style="width:100%;max-width: 10cm;"></v-text-field>
            <v-spacer></v-spacer>

            <v-app-bar-nav-icon @click="drawerRight = !drawerRight"></v-app-bar-nav-icon>
        </v-app-bar>
        <v-main>
            <v-container fluid class="pa-0 ma-0">
                <NuxtPage></NuxtPage>
            </v-container>
        </v-main>
        <v-navigation-drawer floating v-model="drawerRight" location="right" permanent></v-navigation-drawer>
    </v-app>
</template>

<script>
export default {
    data() {
        return {
            drawerLeft: false,
            drawerRight: false,
            navbar: [
                {
                    title:'Home',
                    to:'/',
                    icon:'mdi-home'
                },
                {
                    title:'Courses',
                    to:'tools/',
                    icon:'mdi-book'
                },
                {
                    title:'Tools',
                    to:'tools/',
                    icon:'mdi-hammer-wrench'
                }
            ]
        }
    }
}
</script>

<style lang="scss">
.hdy-searchbar input {
    text-align:center;
}
</style>
