
async function retrieveNews() {
    const searchInput = document.getElementById('cryptoSearch').value.trim();
    if (!searchInput) return;
  
    try {
        const response = await fetch(`/news?crypto_query=${encodeURIComponent(searchInput)}`);
        const newsItems = await response.json();
        
        const resultsContainer = document.getElementById('newsResults');
        resultsContainer.innerHTML = '';
        
        if (newsItems.length === 0) {
            resultsContainer.innerHTML = '<p>No articles found</p>';
            return;
        }
  
        newsItems.forEach(item => {
            const articleElement = document.createElement('article');
            articleElement.className = 'news-card';
            articleElement.innerHTML = `
                <h3 class="article-title">${item.headline}</h3>
                <div class="article-meta">
                    <span class="source">Source: ${item.origin}</span>
                    <time class="date">${new Date(item.published_at).toLocaleDateString()}</time>
                </div>
                <a href="${item.article_url}" class="read-more" target="_blank">Full Article</a>
            `;
            resultsContainer.appendChild(articleElement);
        });
    } catch (error) {
        console.error('Error fetching news:', error);
    }
  }
  