document.addEventListener('DOMContentLoaded', function() {
    const searchBtn = document.getElementById('searchBtn');
    const animeInput = document.getElementById('animeInput');

    searchBtn.addEventListener('click', handleSearch);
    animeInput.addEventListener('keypress', function(e) {
        if (e.key === 'Enter') handleSearch();
    });

    async function handleSearch() {
        const query = animeInput.value.trim();
        
        if (!query) {
            alert('Please enter an anime title');
            return;
        }

        searchBtn.disabled = true;
        searchBtn.textContent = 'Loading...';

        try {
            const response = await fetch(`/api/search?q=${encodeURIComponent(query)}`);
            const data = await response.json();

            displayResults(data.results || []);
        } catch (error) {
            console.error('Error:', error);
            alert('Error fetching recommendations');
        } finally {
            searchBtn.disabled = false;
            searchBtn.textContent = 'Get Recommendations';
        }
    }

    function displayResults(results) {
        const resultsSection = document.getElementById('resultsSection');
        const resultsContainer = document.getElementById('resultsContainer');

        resultsContainer.innerHTML = '';

        if (results.length === 0) {
            resultsContainer.innerHTML = '<p>No recommendations found.</p>';
            resultsSection.style.display = 'block';
            return;
        }

        results.forEach((rec, index) => {
            const card = document.createElement('div');
            card.className = 'recommendation-card';
            card.innerHTML = `
                <h3>${index + 1}. ${rec.title || 'Unknown Anime'}</h3>
                <span class="layer">${rec.layer || 'recommendation'}</span>
                <p class="score">Score: ${(rec.score * 100).toFixed(1)}%</p>
                <p>${rec.explanation || 'See details for more info'}</p>
            `;
            resultsContainer.appendChild(card);
        });

        resultsSection.style.display = 'block';
    }
});
