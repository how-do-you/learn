<template>
    <v-row>
        <v-col cols="12">
            <v-carousel v-model="model">
                <v-carousel-item v-for="(color, i) in colors" :key="color">
                    <v-sheet :color="color" height="100%" tile>                    
                        <div class="d-flex fill-height justify-center align-center">                            
                            <div class="text-h2">
                                Course {{ i + 1 }}
                            </div>
                        </div>
                    </v-sheet>
                </v-carousel-item>
            </v-carousel>
        </v-col>
        <v-col v-for="i in 15" :key="'course'+i" cols="4" lg="3">
            <v-card plain @click="">
                <v-img src="https://www.maxpixel.net/static/photo/1x/Sunset-Nature-Landscape-Travel-Summer-Sun-4672172.jpg"></v-img>
                <v-card-title>Rust for beginners</v-card-title>
                <v-card-text>
                    <p>A good course to start with</p>
                </v-card-text>
            </v-card>
        </v-col>
    </v-row>
</template>

<script>
export default {
    data() {
        return {
            model: 0,
            colors: [
                'orange',
                'secondary',
                'yellow darken-2',
                'red',
                'primary',
            ],
        }
    }
}
</script>
