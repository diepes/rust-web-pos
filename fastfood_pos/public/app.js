// The base URL of our Rust backend API
const API_URL = 'http://localhost:3000';

document.addEventListener('DOMContentLoaded', () => {
    const productsContainer = document.getElementById('products');
    const cartItemsContainer = document.getElementById('cart-items');
    const cartTotalSpan = document.getElementById('cart-total');
    const submitOrderBtn = document.getElementById('submit-order');

    let products = [];
    let cart = new Map(); // Using a Map to store cart items: { productId -> { product, quantity } }

    /**
     * Fetches products from the backend and displays them.
     */
    async function fetchAndDisplayProducts() {
        try {
            const response = await fetch(`${API_URL}/api/products`);
            products = await response.json();

            productsContainer.innerHTML = ''; // Clear existing products
            products.forEach(product => {
                const button = document.createElement('button');
                button.className = 'product-btn';
                button.textContent = `${product.name} - $${product.price.toFixed(2)}`;
                button.onclick = () => addToCart(product);
                productsContainer.appendChild(button);
            });
        } catch (error) {
            console.error('Failed to fetch products:', error);
            productsContainer.innerHTML = '<p>Could not load products. Make sure the server is running.</p>';
        }
    }

    /**
     * Adds a product to the cart or increments its quantity.
     */
    function addToCart(product) {
        if (cart.has(product.id)) {
            cart.get(product.id).quantity++;
        } else {
            cart.set(product.id, { product, quantity: 1 });
        }
        updateCartDisplay();
    }

    /**
     * Updates the cart UI with the current items and total.
     */
    function updateCartDisplay() {
        cartItemsContainer.innerHTML = '';
        let total = 0;

        cart.forEach(item => {
            const li = document.createElement('li');
            li.textContent = `${item.product.name} x ${item.quantity}`;
            cartItemsContainer.appendChild(li);
            total += item.product.price * item.quantity;
        });

        cartTotalSpan.textContent = total.toFixed(2);
    }

    /**
     * Submits the current cart as an order to the backend.
     */
    async function submitOrder() {
        if (cart.size === 0) {
            alert('Your cart is empty!');
            return;
        }

        const orderItems = Array.from(cart.values()).map(item => ({
            product_id: item.product.id,
            quantity: item.quantity,
        }));
        
        const total = parseFloat(cartTotalSpan.textContent);

        try {
            const response = await fetch(`${API_URL}/api/orders`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ items: orderItems, total }),
            });

            if (response.ok) {
                alert('Order submitted successfully!');
                // Clear the cart
                cart.clear();
                updateCartDisplay();
            } else {
                const errorData = await response.json();
                alert(`Failed to submit order: ${JSON.stringify(errorData)}`);
            }
        } catch (error) {
            console.error('Error submitting order:', error);
            alert('An error occurred while submitting the order.');
        }
    }
    
    // Attach event listener for the submit button
    submitOrderBtn.addEventListener('click', submitOrder);

    // Initial load
    fetchAndDisplayProducts();
});
