export class MyChart {
    draw(element_id, config) {
        this.chart = new Chart(
            document.getElementById(element_id),
            JSON.parse(config),
        )
    }
}
