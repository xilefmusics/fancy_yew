export class ChartJsWrapper {
    draw(element_id, config) {
        if (this.chart) {
            this.chart.destroy();
            this.chart = null;
        }
        this.chart = new Chart(
            document.getElementById(element_id),
            JSON.parse(config),
        );
    }
}
