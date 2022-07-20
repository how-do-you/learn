<template>
    <v-app theme="dark">
        <template v-if="splashscreen">
            <v-main>
                <v-container fluid class="fill-height">
                    <v-row align="center" class="fill-height">
                        <v-col align="center">
                            <v-progress-circular :size="150" color="orange" indeterminate>
                                <v-avatar size="75">
                                    <v-img :src="images.icon"></v-img>
                                </v-avatar>
                            </v-progress-circular>
                        </v-col>
                    </v-row>
                </v-container>
            </v-main>
        </template>
        <template v-else>
            <v-navigation-drawer rail v-model="drawerLeft" location="left" permanent expand-on-hover>
                <v-list class="pa-0">
                    <v-list-item v-for="(item, i) in navbar" :key="'nb' + i" :to="item.to">
                        <v-list-item-avatar :color="item.color" start>
                            <v-icon :icon="item.icon"></v-icon>
                        </v-list-item-avatar>
                        <v-list-item-title> {{ item.title }}</v-list-item-title>
                    </v-list-item>
                </v-list>
                <template #append>
                    <v-list class="pa-0">
                        <v-list-item to="/login">
                            <v-list-item-avatar color="white" start>
                                <v-icon>mdi-login</v-icon>
                            </v-list-item-avatar>
                            <v-list-item-title>Log in</v-list-item-title>
                        </v-list-item>
                        <v-list-item to="/login/register">
                            <v-list-item-avatar color="blue" start>
                                <v-icon>mdi-account-plus</v-icon>
                            </v-list-item-avatar>
                            <v-list-item-title>Register</v-list-item-title>
                        </v-list-item>
                    </v-list>
                </template>
            </v-navigation-drawer>
            <v-app-bar density="compact" height="71">
                <v-app-bar-nav-icon @click="drawerLeft = !drawerLeft"></v-app-bar-nav-icon>
                <v-spacer></v-spacer>
                <v-text-field hide-details single-line variant="underlined" placeholder="How do you..." class="hdy-searchbar"
                    style="width:100%;max-width: 15cm;"></v-text-field>
                <v-spacer></v-spacer>

                <v-btn icon="mdi-bell" @click="drawerRight = !drawerRight"> </v-btn>
            </v-app-bar>
            <v-main>
                <v-container fluid class="pa-0 ma-0 fill-height">
                    <NuxtPage></NuxtPage>
                </v-container>
            </v-main>
            <v-navigation-drawer v-model="drawerRight" location="right" permanent floating>
                <v-list class="pa-0" lines="two">
                    <v-list-item @click="">
                        <v-list-item-header>
                            <v-list-item-title>Learn something</v-list-item-title>
                            <v-list-item-subtitle>Learn something</v-list-item-subtitle>
                        </v-list-item-header>
                    </v-list-item>
                </v-list>
            </v-navigation-drawer>
        </template>
    </v-app>
</template>

<script>
import { nextTick } from 'vue'
import icon from '~/assets/icons/128x128.png'
export default {
    data() {
        return {
            images: {
                icon
            },
            splashscreen: true,
            drawerLeft: true,
            drawerRight: false,
            navbar: [
                {
                    title: 'Discover',
                    to: '/',
                    icon: 'mdi-home',
                    color: 'red'
                },
                {
                    title: 'Learn',
                    to: '/learn',
                    icon: 'mdi-graph',
                    color: 'orange'
                },
                {
                    title: 'Courses',
                    to: '/courses',
                    icon: 'mdi-book',
                    color: 'yellow'
                },
                {
                    title: 'Books',
                    to: '/books',
                    icon: 'mdi-book-open-variant',
                    color: 'light-green'
                },
                {
                    title: 'Certifications',
                    to: '/certifications',
                    icon: 'mdi-file-document',
                    color: 'green'
                },
                {
                    title: 'Tools',
                    to: '/tools',
                    icon: 'mdi-hammer-wrench',
                    color: 'teal'
                }
            ]
        }
    },
    async mounted() {
        const self = this
        await nextTick()
        self.splashscreen = false
    }
}
</script>

<style lang="scss">
.hdy-searchbar input {
    text-align: center;
}
</style>
