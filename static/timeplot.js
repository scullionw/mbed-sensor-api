
// bb.generate({
//     bindto: "#chart",
//     data: {
//         columns: [
//             ["data1", 30, 180, 100, 170, 150, 250],
//             ["data2", 130, 100, 140, 35, 110, 50]
//         ],
//         types: {
//           data1: "line",
//           data2: "area-spline"
//         },
//         colors: {
//           data1: "red",
//           data2: "green"
//         }
//     }
// });

function draw(bindaddr, sensor_data) {
	bb.generate({
		bindto: bindaddr,
		data: {
			json: sensor_data,
			keys: {
				x: "year", // it's possible to specify "x" when category axis
				value: ["value"],
            },
            type: "spline",
        },
		axis: {
			x: {
				// type: "category"
			}
		}
	});
};