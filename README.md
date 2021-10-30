
# LaaS: Life as a Service

```shell
$ curl  life-as-a-service.herokuapp.com/-1x0~0x0~1x0
0x-1~0x0~0x1
```

```javascript
let previous = '0x-1~0x0~0x1'
for (let i = 0; i < 5; i++) {
    console.log(previous)
    previous = await fetch(`https://life-as-a-service.herokuapp.com/${previous}`).then(res => res.text())
}

// 0x-1~0x0~0x1
// -1x0~0x0~1x0
// 0x-1~0x0~0x1
// -1x0~0x0~1x0
// 0x-1~0x0~0x1
```

![Screenshot](/screenshot.png)

[https://life-as-a-service.herokuapp.com/html/0x1~1x2~2x0~2x1~2x2](https://life-as-a-service.herokuapp.com/html/0x1~1x2~2x0~2x1~2x2)

A Life state is represented as a series of row/column pairs separated by `~`s,
with each row and column separated from each other by `x`s. Each endpoint 
returns the following state, given some "current" state. The base endpoint 
responds with a string that can be fed directly back into the API. The `/html`
endpoint renders the grid as HTML and provides a link to the next cycle.

## FAQ

### This is very silly!

Yes it is!

### Why did you make it?

Because it was fun!

### Can I use the live version in a project?

Go for it! It's on Heroku's free tier though so no promises about reliability!

