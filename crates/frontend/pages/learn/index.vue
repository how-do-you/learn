<template>
    <v-row justify="center">
        <v-col>
            <v-toolbar density="compact" height="73">
                <v-btn @click="previous">
                    <v-icon>mdi-skip-previous</v-icon>
                    Previous
                </v-btn>
                <v-spacer></v-spacer>
                <v-btn @click="next">
                    Next
                    <v-icon>mdi-skip-next</v-icon>
                </v-btn>
            </v-toolbar>
        </v-col>
        <v-col cols="12" align="center">
            <v-card max-width="15cm">
                <v-img
                    src="https://education.ec.europa.eu/sites/default/files/styles/eac_ratio_16_9_xl/public/2021-12/Planning%20your%20studies.jpg?h=140710cd&itok=EDqz50YE">
                </v-img>
                <v-card-title>{{ queue[index].title }}</v-card-title>
                <v-card-text>
                    <template v-if="queue[index].type === 'fact'">
                        <p v-for="(paragraph, i) in queue[index].text" :key="'fp-' + i" style="text-align:justify">{{
                                paragraph
                        }}</p>
                    </template>
                    <template v-if="queue[index].type === 'question'">
                        <v-btn v-for="(alternative, i) in queue[index].alternatives" :key="'alt-' + i" block
                            :color="queue[index].selected === i + 1 ? 'primary' : 'default'"
                            @click="queue[index].selected = i + 1">
                            <span style="width:100%;text-align:left;">
                                {{ i + 1 }}) {{ alternative.text }}
                            </span>
                        </v-btn>
                        <template v-if="queue[index].selected === queue[index].answer">
                            <v-alert type="success">Correct! 😄</v-alert>
                        </template>
                        <template v-else-if="queue[index].selected === null">
                        </template>
                        <template v-else>
                            <v-alert type="error">Not quite... 😞</v-alert>
                        </template>
                    </template>
                </v-card-text>
            </v-card>
        </v-col>
    </v-row>
</template>

<script>
export default {
    data() {
        return {
            index: 0,
            queue: [
                {
                    type: 'fact',
                    title: 'The sky is blue',
                    text: ['Because I say so']
                },
                {
                    type: 'question',
                    title: 'What colour is the sky?',
                    text: [],
                    selected: null,
                    answer: 1,
                    alternatives: [
                        { value: 1, text: 'Blue' },
                        { value: 2, text: 'White' },
                        { value: 3, text: 'Purple' },
                    ]
                },
                {
                    type: 'fact',
                    title: 'Clouds are white',
                    text: ['Because I say so']
                },
                {
                    type: 'question',
                    title: 'What colour are clouds?',
                    text: [],
                    selected: null,
                    answer: 2,
                    alternatives: [
                        { value: 1, text: 'Blue' },
                        { value: 2, text: 'White' },
                        { value: 3, text: 'Purple' },
                    ]
                },
                {
                    type: 'question',
                    title: 'What colour would a cloudy sky be?',
                    text: [],
                    selected: null,
                    answer: 1,
                    alternatives: [
                        { value: 1, text: 'Blue and white' },
                        { value: 2, text: 'White and green' },
                        { value: 3, text: 'Black and yellow' },
                    ]
                }
            ]
        }
    },
    mounted() {
        const self = this
        window.addEventListener('keydown', self.navigate)
    },
    beforeUnmount() {
        const self = this
        window.removeEventListener('keydown', self.navigate)
    },
    methods: {
        previous(event) {
            if (event.isTrusted) {
                const self = this
                if (self.index > 0) {
                    self.index -= 1
                }
            }
        },
        next(event) {
            if (event.isTrusted) {
                const self = this
                if (self.index < self.queue.length - 1) {
                    self.index += 1
                }
            }
        },
        navigate(event) {
            if (event.isTrusted) {
                const self = this
                if (event.key === 'ArrowLeft' || event.key === 'q') {
                    self.previous(event)
                }
                if (event.key === 'ArrowRight' || event.key === 'e') {
                    self.next(event)
                }
                if (self.queue[self.index].type === 'question') {
                    const keys = [...Array(10).keys()].map(v => v.toString());
                    keys.forEach((val) => {
                        if (val === '0')
                            val = '10'
                        if (parseInt(val) > self.queue[self.index].alternatives.length) {
                            return
                        }
                        if (event.key === val) {
                            self.queue[self.index].selected = parseInt(val)
                        }
                    })
                }
            } 5
        }
    }
}
</script>
