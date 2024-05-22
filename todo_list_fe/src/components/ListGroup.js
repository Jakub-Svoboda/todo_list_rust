import {useEffect, useState} from "react";

function ListGroup() {
    const [items, setItems] = useState(["Ticket 1", "Ticket 2"]);

    useEffect(() => {
        fetch("https://jsonplaceholder.typicode.com/todos")
            .then(res => res.json())
            .then(data => {
                setItems(data);
            });
    }, []);

    return (
        <>
            <h1>Tickets:</h1>
            {items.length === 0 && <p>No tickets</p>}
            <ul className="list-group">
                {items.map((item, index) =>
                    <li className="list-group-item"
                        key={index}
                        onClick={() => console.log("Click")}>
                        {item.title} {/* Assuming the fetched data is an array of objects with a title property */}
                    </li>
                )}
            </ul>
        </>
    )
}

export default ListGroup;
