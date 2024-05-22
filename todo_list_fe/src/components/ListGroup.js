import {useEffect, useState} from "react";

function ListGroup() {
    const [items, setItems] = useState(["Ticket 1", "Ticket 2"]);

    useEffect(() => {
        fetch("http://localhost:8000/api/v1/ticket")
            .then(res => res.json())
            .then(data => {
                setItems(data);
            });
    }, []);

    console.log(items);

    return (
        <>
            <h1>Tickets:</h1>
            {items.length === 0 && <p>No tickets</p>}
            <ul className="list-group">
                {items.map((item, index) =>
                    <li className="list-group-item"
                        key={index}
                        onClick={() => console.log("Click")}>
                        {item.text} {}
                    </li>
                )}
            </ul>
        </>
    )
}

export default ListGroup;
