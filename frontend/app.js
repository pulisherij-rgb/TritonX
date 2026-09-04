const API_URL = "http://127.0.0.1:9000/api/v1/benchmark";

const ctx = document.getElementById('perfChart').getContext('2d');
const perfChart = new Chart(ctx, {
    type: 'bar',
    data: {
        labels: [],
        datasets: [
            { label: 'TritonX Rust (ms)', data: [], backgroundColor: '#2563eb', borderRadius: 4 },
            { label: 'NumPy C-BLAS (ms)', data: [], backgroundColor: '#94a3b8', borderRadius: 4 },
            { label: 'Pure Python (ms)', data: [], backgroundColor: '#f43f5e', borderRadius: 4 }
        ]
    },
    options: {
        responsive: true,
        maintainAspectRatio: false,
        scales: {
            y: {
                type: 'logarithmic',
                grid: { color: '#1e293b' },
                ticks: { color: '#64748b' }
            },
            x: { grid: { display: false }, ticks: { color: '#64748b' } }
        },
        plugins: { legend: { labels: { color: '#94a3b8' } } }
    }
});

async function executeBenchmark() {
    const runBtn = document.getElementById('runBtn');
    const size = parseInt(document.getElementById('sizeSelect').value);
    
    runBtn.disabled = true;
    runBtn.innerText = "Running...";

    try {
        const response = await fetch(API_URL, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ size: size })
        });
        
        const data = await response.json();

        document.getElementById('gflopsDisplay').innerText = `🔥 ${data.rust_gflops} GFLOPS`;

        perfChart.data.labels.push(`${data.matrix_size}`);
        perfChart.data.datasets[0].data.push(data.rust_time_ms);
        perfChart.data.datasets[1].data.push(data.numpy_time_ms);
        perfChart.data.datasets[2].data.push(data.python_time_ms ? data.python_time_ms : 0);
        perfChart.update();

    } catch (err) {
        alert("Make sure server2.py is running on port 9000!");
    } finally {
        runBtn.disabled = false;
        runBtn.innerText = "Execute Benchmark";
    }
}