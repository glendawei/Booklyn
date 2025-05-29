<template>
    <div 
        id="target-div" 
        :style="{ 
            height: `${height}px`, 
            width: `${width}px`
        }"
    />
</template>
<script>
import Progress from 'progressbar.js';
export default {
    props: [
        'color',
        'width',
        'height',
        'progress'
    ],
    mounted() {
        const progressCircle = new Progress.Circle('#target-div', {
            color: this.color,
            duration: 3000,
            easing: 'easeInOut',
            strokeWidth: 5
        });
        progressCircle.setText(`${this.progress * 100}%`);
        progressCircle.animate(this.progress);
    }
}
</script>